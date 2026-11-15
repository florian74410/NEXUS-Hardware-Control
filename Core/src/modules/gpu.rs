use super::sys::*;
use serde_json::{json,Value};
pub fn sample()->Value{
 let mut devices=Vec::new();
 for p in dirs("/sys/class/drm"){
 let card=name(&p);if !card.strip_prefix("card").map_or(false,|s|!s.is_empty()&&s.bytes().all(|b|b.is_ascii_digit())){continue;}
 let d=p.join("device");if !d.exists(){continue;}let mut temp=None;let mut power=None;let mut fan=None;
 for h in dirs(d.join("hwmon")){temp=temp.or_else(||num(h.join("temp1_input")).map(|x|x/1000.0));power=power.or_else(||num(h.join("power1_average")).or_else(||num(h.join("power1_input"))).map(|x|x/1e6));fan=fan.or_else(||num(h.join("fan1_input")));}
 let driver=std::fs::read_link(d.join("driver")).ok().map(|x|name(&x));
 let actual=num(p.join("gt_act_freq_mhz")).or_else(||num(p.join("gt/gt0/rps_act_freq_mhz")));
 let requested=num(p.join("gt_cur_freq_mhz")).or_else(||num(p.join("gt/gt0/rps_cur_freq_mhz")));
 let rc6=num(p.join("power/rc6_residency_ms")).or_else(||num(p.join("gt/gt0/rc6_residency_ms")));
 devices.push(json!({"card":card,"frequency_mhz":actual,"requested_frequency_mhz":requested,"rc6_residency_ms":rc6,"vendor_id":read(d.join("vendor")),"device_id":read(d.join("device")),"driver":driver,"usage_percent":num(d.join("gpu_busy_percent")),"temperature_c":temp,"power_w":power,"fan_rpm":fan,"vram_used_mib":num(d.join("mem_info_vram_used")).map(|x|x/1048576.0),"vram_total_mib":num(d.join("mem_info_vram_total")).map(|x|x/1048576.0)}));
 }
 let has_nvidia=devices.iter().any(|v|v["vendor_id"].as_str()==Some("0x10de")) || dirs("/sys/bus/pci/devices").iter().any(|p|read(p.join("vendor")).as_deref()==Some("0x10de")&&read(p.join("class")).map_or(false,|s|s.starts_with("0x03")));
 let nv=if has_nvidia{super::nvidia::sample()}else{vec![]};
 let first=devices.first().cloned().unwrap_or(json!({}));json!({"usage_percent":first["usage_percent"],"temperature_c":first["temperature_c"],"power_w":first["power_w"],"devices":devices,"nvidia":nv})
}
