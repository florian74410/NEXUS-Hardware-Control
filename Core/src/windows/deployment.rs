use serde_json::{json,Value};
pub fn hardware()->Value{json!({"os":"windows","arch":std::env::consts::ARCH,"version":sysinfo::System::os_version(),"usb":crate::transport::usb::scan().ok()})}
pub fn check()->Value{json!({"hardware":hardware(),"sensor_helper_available":crate::modules::Collector::snapshot()["sensor_helper_available"]})}
pub fn install(_:bool)->Result<(),String>{Err("Use the Windows NHC installer".into())}
pub fn doctor()->Value{json!({"dependencies":check(),"snapshot_available":crate::service::read().is_some(),"usb":crate::transport::usb::scan().ok()})}
