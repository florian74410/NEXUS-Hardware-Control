use super::sys::*;
use serde_json::{json,Value};
use std::{collections::BTreeMap,time::Instant};
#[derive(Default)]pub struct Storage{prev:BTreeMap<String,(u64,u64,Instant)>}
impl Storage{pub fn sample(&mut self)->Value{
 let now=Instant::now();let mut next=BTreeMap::new();let mut disks=Vec::new();
 for p in dirs("/sys/block"){let id=name(&p);if id.starts_with("loop")||id.starts_with("ram")||id.starts_with("zram"){continue;}
 let stat=read(p.join("stat")).unwrap_or_default();let n:Vec<u64>=stat.split_whitespace().filter_map(|x|x.parse().ok()).collect();
 let rates=if n.len()>=7{next.insert(id.clone(),(n[2],n[6],now));self.prev.get(&id).and_then(|(r,w,t)|{let dt=now.duration_since(*t).as_secs_f64();if dt<=0.0{return None;}Some((n[2].checked_sub(*r)? as f64*512.0/dt,n[6].checked_sub(*w)? as f64*512.0/dt))})}else{None};
 let temp=dirs(p.join("device/hwmon")).into_iter().find_map(|h|num(h.join("temp1_input")).map(|x|x/1000.0));
 disks.push(json!({"device":id,"model":read(p.join("device/model")),"capacity_bytes":num(p.join("size")).map(|x|x*512.0),"rotational":num(p.join("queue/rotational")).map(|x|x==1.0),"read_bytes_s":rates.map(|x|x.0),"write_bytes_s":rates.map(|x|x.1),"temperature_c":temp}));
 }self.prev=next;json!({"devices":disks,"root_filesystem_usage_percent":root_usage()})
}}

fn root_usage()->Option<f64>{
 let o=std::process::Command::new("df").args(["-P","-B1","/"]).env("LC_ALL","C").output().ok()?;
 if !o.status.success(){return None;}let text=std::str::from_utf8(&o.stdout).ok()?;
 let row=text.lines().last()?;let value=row.split_whitespace().nth(4)?.strip_suffix('%')?.parse::<f64>().ok()?;
 if value.is_finite()&&value>=0.0{Some(value)}else{None}
}
