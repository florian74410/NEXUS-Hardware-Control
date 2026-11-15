use serde_json::{json,Value};
use std::{path::PathBuf,time::{Duration,SystemTime,UNIX_EPOCH}};
fn path()->Result<PathBuf,String>{let dir=std::env::var_os("XDG_RUNTIME_DIR").ok_or("XDG_RUNTIME_DIR missing; login session required")?;Ok(PathBuf::from(dir).join("nhc-snapshot.json"))}
fn timestamp()->Result<f64,String>{Ok(SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e|e.to_string())?.as_secs_f64())}
pub fn read()->Option<Value>{
 let v:Value=serde_json::from_slice(&std::fs::read(path().ok()?).ok()?).ok()?;
 let age=timestamp().ok()?-v["timestamp"].as_f64()?;
 if v["version"].as_u64()!=Some(1)||!(0.0..=5.0).contains(&age)||!v["data"].is_object(){return None;}Some(v["data"].clone())
}
pub fn run()->Result<(),String>{
 let p=path()?;let mut collector=crate::modules::Collector::default();
 loop{
 let start=std::time::Instant::now();let data=collector.sample();
 let temp=p.with_extension(format!("{}.tmp",std::process::id()));
 let bytes=serde_json::to_vec(&json!({"version":1,"timestamp":timestamp()?,"data":data})).map_err(|e|e.to_string())?;
 #[cfg(unix)] {use std::os::unix::fs::OpenOptionsExt;use std::io::Write;let mut file=std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&temp).map_err(|e|e.to_string())?;file.write_all(&bytes).map_err(|e|e.to_string())?;}
 #[cfg(not(unix))] std::fs::write(&temp,&bytes).map_err(|e|e.to_string())?;
 std::fs::rename(&temp,&p).map_err(|e|e.to_string())?;
 std::thread::sleep(Duration::from_secs(1).saturating_sub(start.elapsed()));
 }
}
pub fn snapshot()->Value{read().unwrap_or_else(crate::modules::Collector::snapshot)}
