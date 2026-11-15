pub mod cpu;
pub mod ram;
pub mod thermal;
pub mod storage;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Default)]
pub struct Collector { cpu: cpu::Cpu }
impl Collector {
    pub fn sample(&mut self) -> Value {
        let (load, mhz) = self.cpu.sample();
        let (used,total) = ram::sample();
        let (temp, label) = thermal::sample();
        json!({"cpu":{"usage_percent":load,"frequency_mhz":mhz,"temperature_c":temp,"temperature_source":label},
            "ram":{"used_mib":used,"total_mib":total},
            "system":{"os":std::env::consts::OS,"arch":std::env::consts::ARCH},
            "gpu":{"usage_percent":null,"temperature_c":null,"power_w":null}})
    }
    pub fn snapshot() -> Value {
        let mut c=Self::default(); c.sample();
        std::thread::sleep(Duration::from_millis(250)); c.sample()
    }
}
