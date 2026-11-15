use super::external;
use serde_json::{json,Value};
fn number(s:&str)->Option<f64>{s.trim().parse::<f64>().ok().filter(|v|v.is_finite())}
pub fn sample()->Vec<Value>{
 let Some(o)=external::run("nvidia-smi",&["--query-gpu=pci.bus_id,utilization.gpu,temperature.gpu,power.draw,memory.used,memory.total,fan.speed,clocks.current.graphics","--format=csv,noheader,nounits"],"2s")else{return vec![]};
 if !o.status.success(){return vec![];}
 String::from_utf8_lossy(&o.stdout).lines().filter_map(|l|{
 let a:Vec<_>=l.split(',').map(str::trim).collect();if a.len()!=8{return None;}
 Some(json!({"pci_bus_id":a[0],"source":"nvidia-smi","usage_percent":number(a[1]),"temperature_c":number(a[2]),"power_w":number(a[3]),"vram_used_mib":number(a[4]),"vram_total_mib":number(a[5]),"fan_percent":number(a[6]),"frequency_mhz":number(a[7])}))
 }).collect()
}
