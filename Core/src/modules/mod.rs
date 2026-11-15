pub mod cpu;
pub mod ram;
pub mod thermal;
pub mod storage;
mod sys;
mod sensors;
mod gpu;
mod power;
mod network;
mod external;
mod smart;
mod nvidia;
use serde_json::{json,Value};
use std::time::{Duration,Instant};
#[derive(Default)]struct Cache{at:Option<Instant>,value:Value}
impl Cache{fn get(&mut self,ms:u64,read:impl FnOnce()->Value)->Value{if self.at.map_or(true,|t|t.elapsed()>=Duration::from_millis(ms)){self.value=read();self.at=Some(Instant::now());}self.value.clone()}}
pub const NAMES:&[&str]=&["cpu","ram","thermal","gpu","power","fan","storage","motherboard","network","system"];
#[derive(Default)]pub struct Collector{cpu:cpu::Cpu,cores:cpu::Cores,power:power::Power,storage:storage::Storage,network:network::Network,sensor_cache:Cache,gpu_cache:Cache,smart_cache:Cache}
impl Collector{
 pub fn sample(&mut self)->Value{
 let (load,mhz)=self.cpu.sample();let (used,total)=ram::sample();let (temp,label)=thermal::sample();let sensors=self.sensor_cache.get(2000,sensors::sample);
 let mut power=self.power.sample();power["hwmon"]=sensors["power"].clone();
 json!({"cpu":{"usage_percent":load,"frequency_mhz":mhz,"temperature_c":temp,"temperature_source":label,"logical_cpus":self.cores.sample()},
 "ram":{"used_mib":used,"total_mib":total,"usage_percent":used.zip(total).filter(|(_,t)|*t>0).map(|(u,t)|100.0*u as f64/t as f64)},
 "thermal":{"sensors":sensors["temperatures"]},"fan":{"sensors":sensors["fans"]},
 "motherboard":{"vendor":sys::read("/sys/class/dmi/id/board_vendor"),"model":sys::read("/sys/class/dmi/id/board_name"),"voltages":sensors["voltages"],"temperatures":sensors["temperatures"]},
 "gpu":self.gpu_cache.get(500,gpu::sample),"power":power,"storage":self.storage.sample(),"network":self.network.sample(),
 "system":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"kernel":sys::read("/proc/sys/kernel/osrelease"),"uptime_seconds":sys::read("/proc/uptime").and_then(|s|s.split_whitespace().next()?.parse::<f64>().ok())}})
 }
 pub fn snapshot()->Value{let mut c=Self::default();c.sample();std::thread::sleep(Duration::from_millis(250));let mut v=c.sample();v["storage"]["smart"]=c.smart_cache.get(60000,smart::sample);v}
}
