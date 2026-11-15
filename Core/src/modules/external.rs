use std::process::{Command,Stdio};
pub fn run(tool:&str,args:&[&str],seconds:&str)->Option<std::process::Output>{
 Command::new("timeout").args(["--kill-after=1s",seconds,tool]).args(args).stdin(Stdio::null()).stderr(Stdio::null()).output().ok()
}
