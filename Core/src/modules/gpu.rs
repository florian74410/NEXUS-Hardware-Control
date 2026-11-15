use super::sys::*;
use serde_json::{json,Value};
pub fn sample()->Value{
 let mut devices=Vec::new();
 for p in dirs("/sys/class/drm"){
 let card=name(&p);if !card.strip_prefix("card").map_or(false,|s|!s.is_empty()&&s.bytes().all(|b|b.is_ascii_digit())){continue;}
 let d=p.join("device");if !d.exists(){continue;}let mut temp=None;let mut power=None;let mut fan=None;
 for h in dirs(d.join("hwmon")){temp=temp.or_else(||num(h.join("temp1_input")).map(|x|x/1000.0));power=power.or_else(||num(h.join("power1_average")).or_else(||num(h.join("power1_input"))).map(|x|x/1e6));fan=fan.or_else(||num(h.join("fan1_input")));}
 let driver=std::fs::read_link(d.join("driver")).ok().map(|x|name(&x));
 devices.push(json!({"card":card,"vendor_id":read(d.join("vendor")),"device_id":read(d.join("device")),"driver":driver,"usage_percent":num(d.join("gpu_busy_percent")),"temperature_c":temp,"power_w":power,"fan_rpm":fan,"vram_used_mib":num(d.join("mem_info_vram_used")).map(|x|x/1048576.0),"vram_total_mib":num(d.join("mem_info_vram_total")).map(|x|x/1048576.0)}));
 }
 let first=devices.first().cloned().unwrap_or(json!({}));json!({"usage_percent":first["usage_percent"],"temperature_c":first["temperature_c"],"power_w":first["power_w"],"devices":devices})
}
