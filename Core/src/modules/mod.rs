pub mod cpu;
pub mod ram;
pub mod thermal;
pub mod storage;
mod sys;
mod sensors;
mod gpu;
mod power;
mod network;
use serde_json::{json,Value};
use std::time::Duration;
pub const NAMES:&[&str]=&["cpu","ram","thermal","gpu","power","fan","storage","motherboard","network","system"];
#[derive(Default)]pub struct Collector{cpu:cpu::Cpu,cores:cpu::Cores,power:power::Power,storage:storage::Storage,network:network::Network}
impl Collector{
 pub fn sample(&mut self)->Value{
 let (load,mhz)=self.cpu.sample();let (used,total)=ram::sample();let (temp,label)=thermal::sample();let sensors=sensors::sample();
 let mut power=self.power.sample();power["hwmon"]=sensors["power"].clone();
 json!({"cpu":{"usage_percent":load,"frequency_mhz":mhz,"temperature_c":temp,"temperature_source":label,"logical_cpus":self.cores.sample()},
 "ram":{"used_mib":used,"total_mib":total,"usage_percent":used.zip(total).filter(|(_,t)|*t>0).map(|(u,t)|100.0*u as f64/t as f64)},
 "thermal":{"sensors":sensors["temperatures"]},"fan":{"sensors":sensors["fans"]},
 "motherboard":{"vendor":sys::read("/sys/class/dmi/id/board_vendor"),"model":sys::read("/sys/class/dmi/id/board_name"),"voltages":sensors["voltages"],"temperatures":sensors["temperatures"]},
 "gpu":gpu::sample(),"power":power,"storage":self.storage.sample(),"network":self.network.sample(),
 "system":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH,"kernel":sys::read("/proc/sys/kernel/osrelease"),"uptime_seconds":sys::read("/proc/uptime").and_then(|s|s.split_whitespace().next()?.parse::<f64>().ok())}})
 }
 pub fn snapshot()->Value{let mut c=Self::default();c.sample();std::thread::sleep(Duration::from_millis(250));c.sample()}
}
