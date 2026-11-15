use std::path::{Path,PathBuf};
pub fn read(p:impl AsRef<Path>)->Option<String>{Some(std::fs::read_to_string(p).ok()?.trim().to_owned())}
pub fn num(p:impl AsRef<Path>)->Option<f64>{read(p)?.parse::<f64>().ok().filter(|v|v.is_finite())}
pub fn dirs(p:impl AsRef<Path>)->Vec<PathBuf>{let mut v:Vec<_>=std::fs::read_dir(p).into_iter().flatten().flatten().map(|e|e.path()).collect();v.sort();v}
pub fn name(p:&Path)->String{p.file_name().unwrap_or_default().to_string_lossy().into_owned()}
