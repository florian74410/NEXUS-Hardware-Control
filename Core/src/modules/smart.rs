use super::{sys::*,external};
use serde_json::{json,Value};
pub fn sample()->Value{
 let mut disks=Vec::new();
 for p in dirs("/sys/block"){
 let id=name(&p);if !(id.starts_with("sd")||id.starts_with("nvme")||id.starts_with("hd")){continue;}
 let dev=format!("/dev/{id}");
 let Some(o)=external::run("smartctl",&["-j","-a","-n","standby",&dev],"3s") else{disks.push(json!({"device":id,"status":"unavailable"}));continue;};
 let code=o.status.code();let parsed=serde_json::from_slice::<Value>(&o.stdout).ok();
 if let Some(v)=parsed.filter(|_|code.map_or(false,|c|c>=0&&c<128&&(c&7)==0)){
 disks.push(json!({"device":id,"status":"read","healthy":v["smart_status"]["passed"],"temperature_c":v["temperature"]["current"],"percentage_used":v["nvme_smart_health_information_log"]["percentage_used"],"power_on_hours":v["power_on_time"]["hours"],"ata_attributes":v["ata_smart_attributes"]["table"]}));
 }else{disks.push(json!({"device":id,"status":"unavailable_or_standby","exit_code":code}));}
 }json!(disks)
}
