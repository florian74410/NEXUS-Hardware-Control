use crate::{engine, transport::usb};
use serde_json::{json, Value};

const HELP: &str = "NHC Core — CLI\n\n  nhc status [--json]\n  nhc modules list [--json]\n  nhc usb scan [--json]\n  nhc usb ping --port PORT [--json]\n  nhc data get [--json]\n  nhc cpu get [--json]\n  nhc MODULE get [--json]\n  Modules: cpu ram thermal gpu power fan storage motherboard network system\n  nhc usb stream --port PORT\n  nhc --version\n  nhc --help\n\nScan never opens a port. Ping opens only the explicitly selected port. Do not ping while flashing or backing up.";

fn output(v: Value, json_mode: bool) {
    if json_mode { println!("{}", serde_json::to_string(&v).unwrap()); }
    else { println!("{}", serde_json::to_string_pretty(&v).unwrap()); }
}

pub fn run(mut args: Vec<String>) -> Result<(), String> {
    let json_mode = args.iter().any(|a| a == "--json");
    args.retain(|a| a != "--json");
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        [] | ["--help"] | ["help"] => println!("{HELP}"),
        ["--version"] => println!("nhc {}", env!("CARGO_PKG_VERSION")),
        ["status"] => output(engine::status(&[&usb::UsbModule]),json_mode),
        ["modules","list"] => output(json!({"implemented":crate::modules::NAMES,"transport":["usb"]}),json_mode),
        ["usb","scan"] => output(json!({"candidates":usb::scan()?}),json_mode),
        ["usb","ping","--port",path] => output(usb::ping(path)?,json_mode),
        ["data","get"] => output(crate::modules::Collector::snapshot(),json_mode),
        ["cpu","get"] => output(crate::modules::Collector::snapshot()["cpu"].clone(),json_mode),
        ["ram","get"] => output(crate::modules::Collector::snapshot()["ram"].clone(),json_mode),
        [module,"get"] if crate::modules::NAMES.contains(module) => output(crate::modules::Collector::snapshot()[*module].clone(),json_mode),
        ["usb","stream","--port",path] => usb::stream(path)?,
        _ => return Err(format!("Unknown command. Run nhc --help")),
    }
    Ok(())
}
