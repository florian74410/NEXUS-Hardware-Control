use super::sys::*;
use serde_json::{json,Value};
use std::{collections::BTreeMap,time::Instant};
#[derive(Default)]pub struct Network{prev:BTreeMap<String,(u64,u64,Instant)>}
impl Network{pub fn sample(&mut self)->Value{
 let now=Instant::now();let mut next=BTreeMap::new();let mut interfaces=Vec::new();
 for p in dirs("/sys/class/net"){let id=name(&p);if id=="lo"{continue;}
 let get=|k|read(p.join("statistics").join(k)).and_then(|s|s.parse::<u64>().ok());
 let (rx,tx)=(get("rx_bytes"),get("tx_bytes"));
 let rate=rx.zip(tx).and_then(|(r,w)|{next.insert(id.clone(),(r,w,now));let (a,b,t)=self.prev.get(&id)?;let dt=now.duration_since(*t).as_secs_f64();if dt<=0.0{return None;}Some((r.checked_sub(*a)? as f64/dt,w.checked_sub(*b)? as f64/dt))});
 interfaces.push(json!({"interface":id,"state":read(p.join("operstate")),"rx_bytes":rx,"tx_bytes":tx,"rx_bytes_s":rate.map(|x|x.0),"tx_bytes_s":rate.map(|x|x.1)}));
 }self.prev=next;json!({"interfaces":interfaces})
}}
