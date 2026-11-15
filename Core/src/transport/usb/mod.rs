use crate::engine::Module;
use serde::Serialize;
use serde_json::{json, Value};
use serialport::{SerialPortInfo, SerialPortType};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const BAUD: u32 = 115_200;
const MAX_FRAME: usize = 4096;

#[derive(Debug, Serialize)]
pub struct Candidate {
    pub port: String,
    pub vid: u16,
    pub pid: u16,
    pub identity: &'static str,
}

fn candidate(info: SerialPortInfo) -> Option<Candidate> {
    if let SerialPortType::UsbPort(usb) = info.port_type {
        if usb.vid == 0x1a86 && usb.pid == 0x7523 {
            return Some(Candidate { port: info.port_name, vid: usb.vid, pid: usb.pid,
                identity: "CH340 candidate; NHC identity unverified" });
        }
    }
    None
}

pub fn scan() -> Result<Vec<Candidate>, String> {
    let mut devices: Vec<_> = serialport::available_ports().map_err(|e| e.to_string())?
        .into_iter().filter_map(candidate).collect();
    devices.sort_by(|a,b| a.port.cmp(&b.port));
    Ok(devices)
}

pub struct UsbModule;
impl Module for UsbModule {
    fn name(&self) -> &'static str { "usb" }
    fn status(&self) -> Result<Value, String> { Ok(json!({"candidates":scan()?})) }
}

fn response(line: &[u8], kind: &str, nonce: &str) -> Option<Value> {
    let v: Value = serde_json::from_slice(line).ok()?;
    if v["type"].as_str() == Some(kind) && v["version"].as_u64() == Some(1)
        && v["nonce"].as_str() == Some(nonce) && v["device"].as_str() == Some("nhc-touch") {
        Some(v)
    } else { None }
}

fn exchange(port: &mut dyn serialport::SerialPort, request: Value, kind: &str, nonce: &str)
    -> Result<Value, String> {
    let deadline = Instant::now() + Duration::from_secs(4);
    let mut line = Vec::new();
    let mut overflow = false;
    let mut next_tx = Instant::now();
    let mut bytes = serde_json::to_vec(&request).map_err(|e|e.to_string())?;
    if bytes.len()>MAX_FRAME {return Err("USB frame exceeds protocol limit".into());}
    bytes.push(b'\n');
    while Instant::now() < deadline {
        if Instant::now() >= next_tx {
            port.write_all(&bytes).map_err(|e|e.to_string())?;
            next_tx = Instant::now() + Duration::from_millis(750);
        }
        let mut buf = [0u8; 256];
        match port.read(&mut buf) {
            Ok(n) => for &b in &buf[..n] {
                if b == b'\n' {
                    if !overflow {
                        if let Some(v) = response(&line, kind, nonce) { return Ok(v); }
                    }
                    line.clear(); overflow = false;
                } else if !overflow {
                    if line.len() < MAX_FRAME { line.push(b); }
                    else { line.clear(); overflow = true; }
                }
            },
            Err(e) if matches!(e.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::Interrupted) => {},
            Err(e) => return Err(e.to_string()),
        }
    }
    Err(format!("No valid {kind} reply; NHC Touch firmware required"))
}

pub fn ping(path: &str) -> Result<Value, String> {
    let mut port = serialport::new(path, BAUD).timeout(Duration::from_millis(100))
        .open().map_err(|e|format!("{path}: {e}"))?;
    // The bridge may reset the ESP32 on open. Allow boot before sending.
    std::thread::sleep(Duration::from_millis(1200));
    let nonce = format!("{}-{}", std::process::id(),
        SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos());
    let hello = exchange(&mut *port, json!({"type":"hello","version":1,"nonce":nonce}), "hello_ack", &nonce)?;
    let start = Instant::now();
    let pong = exchange(&mut *port, json!({"type":"ping","version":1,"nonce":nonce}), "pong", &nonce)?;
    Ok(json!({"port":path,"baud":BAUD,"verified":true,"hello":hello,"pong":pong,"round_trip_ms":start.elapsed().as_millis()}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_and_nonce_required() {
        let valid = br#"{"type":"pong","version":1,"nonce":"abc","device":"nhc-touch"}"#;
        assert!(response(valid,"pong","abc").is_some());
        assert!(response(valid,"pong","wrong").is_none());
        assert!(response(valid,"hello_ack","abc").is_none());
        assert!(response(b"boot log","pong","abc").is_none());
        assert!(response(br#"{"type":"pong","version":1,"nonce":"abc","device":"other"}"#,"pong","abc").is_none());
    }
}

fn stream_once(path:&str)->Result<(),String>{
    let mut port=serialport::new(path,BAUD).timeout(Duration::from_millis(100)).open().map_err(|e|e.to_string())?;
    std::thread::sleep(Duration::from_millis(1200));
    let nonce=format!("stream-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_nanos());
    let hello=exchange(&mut *port,json!({"type":"hello","version":1,"nonce":nonce}),"hello_ack",&nonce)?;
    if hello["telemetry"].as_bool()!=Some(true){return Err("Touch firmware does not support telemetry; upgrade Touch first".into());}
    eprintln!("NHC Touch verified; streaming every 1000 ms (Ctrl+C to stop)");
    let mut collector=crate::modules::Collector::default();
    collector.sample();
    std::thread::sleep(Duration::from_millis(250));
    let mut seq=0u64;
    loop {
        seq+=1;
        let data=crate::service::read().unwrap_or_else(||collector.sample());
        let ack_nonce=format!("{nonce}-{seq}");
        let compact=crate::projection::compact(&data,hello["telemetry_extended"].as_bool()==Some(true));
        let req=json!({"type":"telemetry","version":1,"nonce":ack_nonce,"session":nonce,"seq":seq,"data":compact});
        exchange(&mut *port,req,"telemetry_ack",&ack_nonce)?;
        println!("{}",serde_json::to_string(&compact).map_err(|e|e.to_string())?);
        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn stream(path:&str)->Result<(),String>{
 // Explicit path only: never probe unrelated serial devices.
 loop {if let Err(e)=stream_once(path){eprintln!("USB offline: {e}; retry in 3s");}
 std::thread::sleep(Duration::from_secs(3));}
}
