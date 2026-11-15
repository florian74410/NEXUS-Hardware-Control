use serde_json::{json,Value};
use std::{fs,path::Path,process::Command};
fn read(p:&str)->String{fs::read_to_string(p).unwrap_or_default().trim().to_owned()}
fn has(cmd:&str)->bool{std::env::var_os("PATH").map_or(false,|p|std::env::split_paths(&p).any(|d|d.join(cmd).is_file()))}
fn distro()->String{read("/etc/os-release").lines().find_map(|l|l.strip_prefix("ID=").map(|s|s.trim_matches('"').to_owned())).unwrap_or_default()}
fn manager()->&'static str{if Path::new("/run/ostree-booted").exists(){"immutable"}else if has("apt-get"){"apt"}else if has("dnf"){"dnf"}else{"unsupported"}}
pub fn hardware()->Value{
 let cpu=read("/proc/cpuinfo");let apple=read("/sys/class/dmi/id/sys_vendor").to_lowercase().contains("apple");
 let mut vendors=Vec::new();
 for p in fs::read_dir("/sys/bus/pci/devices").into_iter().flatten().flatten(){let p=p.path();let c=fs::read_to_string(p.join("class")).unwrap_or_default();if c.starts_with("0x03"){let v=fs::read_to_string(p.join("vendor")).unwrap_or_default().trim().to_owned();if !vendors.contains(&v){vendors.push(v);}}}
 let disks:Vec<_>=fs::read_dir("/sys/block").into_iter().flatten().flatten().map(|e|e.file_name().to_string_lossy().into_owned()).filter(|s|s.starts_with("sd")||s.starts_with("nvme")||s.starts_with("hd")).collect();
 json!({"os":std::env::consts::OS,"distribution":distro(),"manager":manager(),"apple":apple,"intel_cpu":cpu.contains("GenuineIntel"),"amd_cpu":cpu.contains("AuthenticAMD"),"gpu_vendors":vendors,"disks":disks})
}
fn plan(h:&Value)->Vec<(&'static str,&'static str,&'static str)>{
 let mut p=vec![("timeout","coreutils","coreutils"),("sensors","lm-sensors","lm_sensors")];
 if h["disks"].as_array().map_or(false,|a|!a.is_empty()){p.push(("smartctl","smartmontools","smartmontools"));}
 if h["gpu_vendors"].as_array().map_or(false,|a|a.iter().any(|v|v.as_str()==Some("0x8086"))){p.push(("intel_gpu_top","intel-gpu-tools","igt-gpu-tools"));}
 p
}
fn modules(h:&Value)->Vec<&'static str>{let mut v=Vec::new();if h["apple"]==true{v.push("applesmc");}if h["intel_cpu"]==true{v.push("coretemp");}if h["amd_cpu"]==true{v.push("k10temp");}v}
pub fn check()->Value{let h=hardware();let deps:Vec<_>=plan(&h).iter().map(|(tool,apt,dnf)|json!({"tool":tool,"package":if manager()=="dnf"{dnf}else{apt},"installed":has(tool)})).collect();let drivers:Vec<_>=modules(&h).iter().map(|m|json!({"module":m,"loaded":Path::new("/sys/module").join(m).exists()})).collect();let nv=h["gpu_vendors"].as_array().map_or(false,|a|a.iter().any(|v|v.as_str()==Some("0x10de")));json!({"hardware":h,"dependencies":deps,"kernel_modules":drivers,"nvidia_smi":if nv{Some(has("nvidia-smi"))}else{None}})}
fn privileged(program:&str,args:&[&str])->Result<(),String>{let status=Command::new("sudo").arg(program).args(args).status().map_err(|e|e.to_string())?;if status.success(){Ok(())}else{Err(format!("{program} failed: {status}"))}}
pub fn install(yes:bool)->Result<(),String>{
 if !cfg!(target_os="linux"){return Err("Linux only".into());}
 let h=hardware();let mgr=manager();if mgr!="apt"&&mgr!="dnf"{return Err(format!("Package installation unsupported on {mgr}; use host-native procedure"));}
 let mut packages=Vec::new();for (tool,a,d) in plan(&h){if has(tool){println!("[OK] {tool}");}else{let pkg=if mgr=="apt"{a}else{d};println!("[MISSING] {tool}: {pkg}");packages.push(pkg);}}
 let drivers:Vec<_>=modules(&h).into_iter().filter(|m|!Path::new("/sys/module").join(m).exists()).collect();
 if !yes&&(!packages.is_empty()||!drivers.is_empty()){return Err("Review plan and rerun dependencies install --yes to authorize sudo".into());}
 if !packages.is_empty(){if mgr=="apt"{privileged("apt-get",&["update"])?;let mut a=vec!["install","-y"];a.extend(packages);privileged("apt-get",&a)?;}else{let mut a=vec!["install","-y"];a.extend(packages);privileged("dnf",&a)?;}}
 for m in drivers{if let Err(e)=privileged("modprobe",&[m]){eprintln!("[WARN] {m}: {e}");}else{println!("[OK] {m} loaded");}}
 for (tool,_,_) in plan(&h){println!("[{}] {tool}",if has(tool){"OK"}else{"MISSING"});}
 if h["gpu_vendors"].as_array().map_or(false,|a|a.iter().any(|v|v.as_str()==Some("0x10de")))&&!has("nvidia-smi"){eprintln!("[WARN] NVIDIA utilities absent; use matching distribution driver package; not auto-replaced");}
 Ok(())
}
pub fn doctor()->Value{let data=crate::modules::Collector::snapshot();json!({"dependencies":check(),"rapl_source":data["power"]["rapl_source"],"rapl_domains":data["power"]["rapl_domains"],"rapl_inaccessible":data["power"]["rapl_inaccessible"],"smart":data["storage"]["smart"],"fan_count":data["fan"]["sensors"].as_array().map(|a|a.len()),"temperature_count":data["thermal"]["sensors"].as_array().map(|a|a.len()),"usb":crate::transport::usb::scan().ok()})}
