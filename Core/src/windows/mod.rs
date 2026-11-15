use serde_json::{json,Value};
use sysinfo::{System,Networks,Disks};
use std::{path::PathBuf,time::{Instant,Duration,SystemTime,UNIX_EPOCH}};
pub const NAMES:&[&str]=&["cpu","ram","thermal","gpu","power","fan","storage","motherboard","network","system"];
pub struct Collector{sys:System,nets:Networks,disks:Disks,last:Option<Instant>,disk_at:Option<Instant>}
impl Default for Collector{fn default()->Self{Self{sys:System::new(),nets:Networks::new_with_refreshed_list(),disks:Disks::new_with_refreshed_list(),last:None,disk_at:None}}}
fn sensors()->Value{
 let Some(p)=std::env::var_os("LOCALAPPDATA") else{return Value::Null};
 let p=PathBuf::from(p).join("NEXUS-Hardware-Control").join("state").join("sensors.json");
 let Ok(b)=std::fs::read(p) else{return Value::Null};
 let Ok(v)=serde_json::from_slice::<Value>(&b) else{return Value::Null};
 let now=SystemTime::now().duration_since(UNIX_EPOCH).map(|x|x.as_secs_f64()).unwrap_or(0.0);
 if v["version"]==1 && v["timestamp"].as_f64().is_some_and(|t|(0.0..=10.0).contains(&(now-t))){v["data"].clone()}else{Value::Null}
}
impl Collector{
 pub fn sample(&mut self)->Value{
 let dt=self.last.replace(Instant::now()).map(|t|t.elapsed().as_secs_f64()).filter(|t|*t>=0.1);
 self.sys.refresh_cpu_all();self.sys.refresh_memory();self.nets.refresh(true);
 if self.disk_at.map_or(true,|t|t.elapsed()>=Duration::from_secs(5)){self.disks.refresh(true);self.disk_at=Some(Instant::now());}
 let s=sensors();let total=self.sys.total_memory();let used=self.sys.used_memory();
 let freq=self.sys.cpus().first().map(|c|c.frequency()).filter(|v|*v>0);
 let load=dt.map(|_|self.sys.global_cpu_usage());
 let root=std::env::var("SystemDrive").unwrap_or_else(|_|"C:".into()).to_lowercase();
 let disk=self.disks.list().iter().find(|d|d.mount_point().to_string_lossy().to_lowercase().starts_with(&root));
 let usage=disk.filter(|d|d.total_space()>0).map(|d|100.0*(d.total_space().saturating_sub(d.available_space())) as f64/d.total_space() as f64);
 let network:Vec<_>=self.nets.iter().filter(|(n,_)|!n.to_lowercase().contains("loopback")).map(|(n,d)|json!({"interface":n,"rx_bytes_s":dt.map(|t|d.received() as f64/t),"tx_bytes_s":dt.map(|t|d.transmitted() as f64/t)})).collect();
 // Aggregate non-loopback adapters; virtual interfaces may be included.
 let sum=|key:&str|dt.map(|_|network.iter().filter_map(|n|n[key].as_f64()).sum::<f64>());
 let cpu=json!({"usage_percent":load,"frequency_mhz":freq,"temperature_c":s["cpu_temp"],"logical_cpus":self.sys.cpus().iter().map(|c|json!({"usage_percent":c.cpu_usage(),"frequency_mhz":c.frequency()})).collect::<Vec<_>>()});
 let ram=json!({"used_mib":used/1048576,"total_mib":total/1048576,"usage_percent":if total>0{Some(100.0*used as f64/total as f64)}else{None}});
 let power=json!({"cpu_package_w":s["cpu_power"],"system_total_w":null});
 let storage=json!({"root_filesystem_usage_percent":usage,"capacity_bytes":disk.map(|d|d.total_space()),"temperature_c":null,"healthy":null,"read_bytes_s":null,"write_bytes_s":null});
 let thermal=json!({"sensors":s["temperatures"],"count":s["temperatures"].as_array().map(|a|a.len()),"max_c":s["temperatures"].as_array().into_iter().flatten().filter_map(|a|a["value"].as_f64()).reduce(f64::max)});
 let fan=json!({"rpm":s["fan_rpm"],"sensors":s["fans"]});
 let board=json!({"voltage_count":s["voltages"].as_array().map(|a|a.len()),"voltages":s["voltages"]});
 let system=json!({"os":"windows","arch":std::env::consts::ARCH,"uptime_seconds":System::uptime()});
 let gpu=s["gpu"].clone();let net=json!({"rx_bytes_s":sum("rx_bytes_s"),"tx_bytes_s":sum("tx_bytes_s"),"interfaces":network});
 let battery=json!({"capacity_percent":null});
 json!({"cpu":cpu,"ram":ram,"power":power,"storage":storage,"thermal":thermal,"fan":fan,"motherboard":board,"system":system,"gpu":gpu,"network":net,"battery":battery,
 "windows_touch":{"power":power,"storage":storage,"thermal":{"count":thermal["count"],"max_c":thermal["max_c"]},"fan":{"rpm":fan["rpm"]},"motherboard":{"voltage_count":board["voltage_count"]},"system":{"uptime_seconds":system["uptime_seconds"]},"gpu":gpu,"network":{"rx_bytes_s":net["rx_bytes_s"],"tx_bytes_s":net["tx_bytes_s"]},"battery":battery},"sensor_helper_available":!s.is_null()})
 }
 pub fn snapshot()->Value{let mut c=Self::default();c.sample();std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL+Duration::from_millis(100));c.sample()}
}
