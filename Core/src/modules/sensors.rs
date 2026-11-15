use super::sys::*;
use serde_json::{json,Value};
pub fn sample()->Value{
 let(mut temps,mut fans,mut volts,mut powers)=(Vec::new(),Vec::new(),Vec::new(),Vec::new());
 let mut paths=dirs("/sys/class/hwmon");
 for p in dirs("/sys/devices/platform"){if name(&p).starts_with("applesmc"){paths.push(p);}}
 let mut seen=std::collections::BTreeSet::new();
 for p in paths{
 let chip=read(p.join("name")).unwrap_or_else(||name(&p));
 for q in dirs(&p){let file=name(&q);let Some(stem)=file.strip_suffix("_input").or_else(||file.strip_suffix("_average")) else{continue};
 let key=std::fs::canonicalize(&q).unwrap_or(q.clone());if !seen.insert(key){continue;}let Some(v)=num(&q) else{continue};let label=read(p.join(format!("{stem}_label"))).unwrap_or_else(||stem.into());
 let row=|value:f64,unit:&str|json!({"chip":chip,"label":label,"source":q,"value":value,"unit":unit});
 if stem.starts_with("temp"){temps.push(row(v/1000.0,"C"));}
 else if stem.starts_with("fan"){fans.push(row(v,"rpm"));}
 else if stem.starts_with("in"){volts.push(row(v/1000.0,"V"));}
 else if stem.starts_with("power"){powers.push(row(v/1_000_000.0,"W"));}
 }
 }json!({"temperatures":temps,"fans":fans,"voltages":volts,"power":powers})
}
