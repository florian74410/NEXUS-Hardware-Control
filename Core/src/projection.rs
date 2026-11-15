use serde_json::{json,Value};
fn rows(v:&Value)->impl Iterator<Item=&Value>{v.as_array().into_iter().flatten()}
fn first(v:&Value)->Value{rows(v).next().cloned().unwrap_or(Value::Null)}
fn sum(v:&Value,key:&str)->Option<f64>{let mut it=rows(v).filter_map(|x|x[key].as_f64());let a=it.next()?;Some(it.fold(a,|a,b|a+b))}
pub fn compact(d:&Value,extended:bool)->Value{
 let mut v=json!({"cpu":{"usage_percent":d["cpu"]["usage_percent"],"frequency_mhz":d["cpu"]["frequency_mhz"],"temperature_c":d["cpu"]["temperature_c"]},"ram":{"used_mib":d["ram"]["used_mib"],"total_mib":d["ram"]["total_mib"]}});
 if !extended{return v;}
 let gpu=rows(&d["gpu"]["nvidia"]).next().cloned().unwrap_or_else(||first(&d["gpu"]["devices"]));
 let disk=rows(&d["storage"]["devices"]).find(|x|x["device"].as_str().map_or(false,|s|s.starts_with("sd")||s.starts_with("nvme")||s.starts_with("hd"))).cloned().unwrap_or(Value::Null);
 let smart=rows(&d["storage"]["smart"]).find(|x|!disk["device"].is_null()&&x["device"]==disk["device"]).cloned().unwrap_or(Value::Null);
 let net=rows(&d["network"]["interfaces"]).find(|x|x["state"]=="up"&&x["interface"].as_str().map_or(false,|s|s.starts_with("en")||s.starts_with("wl")||s.starts_with("eth"))).cloned().unwrap_or(Value::Null);
 let packages:Vec<_>=rows(&d["power"]["rapl_domains"]).filter(|x|x["name"].as_str().map_or(false,|s|s.starts_with("package"))).cloned().collect();
 v["power"]=json!({"cpu_package_w":sum(&json!(packages),"power_w"),"system_total_w":d["power"]["system_total_w"]});
 v["gpu"]=json!({"usage_percent":gpu["usage_percent"],"non_rc6_percent":gpu["non_rc6_percent"],"temperature_c":gpu["temperature_c"],"frequency_mhz":gpu["frequency_mhz"],"power_w":gpu["power_w"],"vram_used_mib":gpu["vram_used_mib"],"vram_total_mib":gpu["vram_total_mib"]});
 v["fan"]=json!({"rpm":first(&d["fan"]["sensors"])["value"]});
 v["storage"]=json!({"capacity_bytes":disk["capacity_bytes"],"percentage_used":smart["percentage_used"],"temperature_c":smart["temperature_c"].as_f64().or_else(||disk["temperature_c"].as_f64()),"healthy":smart["healthy"],"read_bytes_s":disk["read_bytes_s"],"write_bytes_s":disk["write_bytes_s"]});
 v["network"]=json!({"rx_bytes_s":net["rx_bytes_s"],"tx_bytes_s":net["tx_bytes_s"]});
 v["thermal"]=json!({"count":d["thermal"]["sensors"].as_array().map(|a|a.len()),"max_c":rows(&d["thermal"]["sensors"]).filter_map(|x|x["value"].as_f64()).reduce(f64::max)});
 v["motherboard"]=json!({"voltage_count":d["motherboard"]["voltages"].as_array().map(|a|a.len())});
 v["system"]=json!({"uptime_seconds":d["system"]["uptime_seconds"]});
 v["battery"]=json!({"capacity_percent":first(&d["power"]["batteries"])["capacity_percent"]});
 v
}
#[cfg(test)]mod tests{use super::*;#[test]fn nulls_and_package_only(){let d=json!({"power":{"rapl_domains":[{"name":"package-0","power_w":10.0},{"name":"core","power_w":6.0}]}});
 let v=compact(&d,true);assert_eq!(v["power"]["cpu_package_w"],10.0);assert!(v["gpu"]["temperature_c"].is_null());assert!(compact(&d,false)["power"].is_null());assert!(serde_json::to_vec(&v).unwrap().len()<3000);}}
