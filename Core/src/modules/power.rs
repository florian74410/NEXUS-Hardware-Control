use super::sys::*;
use serde_json::{json,Value};
use std::{collections::BTreeMap,time::Instant,path::Path};
#[derive(Default)]pub struct Power{prev:BTreeMap<String,(f64,Instant)>}
impl Power{
 pub fn sample(&mut self)->Value{
 let mut domains=Vec::new();let mut inaccessible=Vec::new();let now=Instant::now();let mut next=BTreeMap::new();
 fn visit(p:&Path,depth:u8,paths:&mut Vec<std::path::PathBuf>){if depth>3{return;}for q in dirs(p){if name(&q).contains("rapl")&&q.is_dir(){paths.push(q.clone());visit(&q,depth+1,paths);}}}
 let mut paths=Vec::new();visit(Path::new("/sys/class/powercap"),0,&mut paths);let mut seen=std::collections::BTreeSet::new();
 for p in paths{let key=std::fs::canonicalize(&p).unwrap_or(p.clone()).to_string_lossy().into_owned();if !seen.insert(key.clone()){continue;}
 let Some(e)=num(p.join("energy_uj")) else{if p.join("energy_uj").exists(){inaccessible.push(p.clone());}continue;};let max=num(p.join("max_energy_range_uj"));
 let watts=self.prev.get(&key).and_then(|(old,t)|{let dt=now.duration_since(*t).as_secs_f64();let delta=if e>=*old{e-*old}else{max? - *old+e};if dt>0.0&&delta>=0.0{Some(delta/1e6/dt)}else{None}});
 next.insert(key,(e,now));domains.push(json!({"name":read(p.join("name")),"source":p,"energy_uj":e,"power_w":watts}));
 }self.prev=next;
 let batteries:Vec<_>=dirs("/sys/class/power_supply").into_iter().filter(|p|read(p.join("type")).as_deref()==Some("Battery")).map(|p|json!({"name":name(&p),"capacity_percent":num(p.join("capacity")),"status":read(p.join("status")),"power_w":num(p.join("power_now")).map(|x|x/1e6).or_else(||Some(num(p.join("voltage_now"))?*num(p.join("current_now"))?/1e12))})).collect();
 let helper=super::helper::read("rapl",5.0);
 let source=if helper.is_some(){"helper"}else{"direct"};
 if let Some(v)=helper{if let Some(rows)=v.as_array(){domains=rows.clone();inaccessible.retain(|p|{let canonical=std::fs::canonicalize(p).ok();!domains.iter().any(|d|canonical.as_ref().map(|p|p.to_string_lossy().as_ref()==d["source"].as_str().unwrap_or("")).unwrap_or(false))});}}
 json!({"rapl_source":source,"rapl_domains":domains,"rapl_inaccessible":inaccessible,"batteries":batteries,"system_total_w":null})
 }
}
