use serde_json::Value;
use std::time::{SystemTime,UNIX_EPOCH};
pub fn read(kind:&str,max_age:f64)->Option<Value>{
 let path=match kind{"rapl"=>"/run/nhc-sensors/rapl.json","smart"=>"/run/nhc-sensors/smart.json",_=>return None};
 let v:Value=serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()?;
 let now=SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs_f64();
 let age=now-v["timestamp"].as_f64()?;
 if v["version"].as_u64()!=Some(1)||age<0.0||age>max_age||!v["data"].is_array(){return None;}
 Some(v["data"].clone())
}
