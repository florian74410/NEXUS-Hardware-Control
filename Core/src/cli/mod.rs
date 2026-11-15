use crate::{engine, transport::usb};
use serde_json::{json, Value};

const HELP: &str = "NHC Core — CLI\n\n  nhc status [--json]\n  nhc modules list [--json]\n  nhc usb scan [--json]\n  nhc usb ping --port PORT [--json]\n  nhc data get [--json]\n  nhc cpu get [--json]\n  nhc MODULE get [--json]\n  Modules: cpu ram thermal gpu power fan storage motherboard network system\n  nhc usb stream --port PORT\n  nhc usb auto\n  nhc hardware scan\n  nhc dependencies check\n  nhc dependencies install [--yes]\n  nhc doctor\n  nhc service run\n  nhc service status\n  nhc data touch [--json]\n  nhc --version\n  nhc --help\n\nScan never opens a port. Ping opens only the explicitly selected port. Do not ping while flashing or backing up.";

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
        ["hardware","scan"] => output(crate::deployment::hardware(),json_mode),
        ["dependencies","check"] => output(crate::deployment::check(),json_mode),
        ["dependencies","install"] => crate::deployment::install(false)?,
        ["dependencies","install","--yes"] => crate::deployment::install(true)?,
        ["doctor"] => output(crate::deployment::doctor(),json_mode),
        ["status"] => output(engine::status(&[&usb::UsbModule]),json_mode),
        ["modules","list"] => output(json!({"implemented":crate::modules::NAMES,"transport":["usb"]}),json_mode),
        ["usb","scan"] => output(json!({"candidates":usb::scan()?}),json_mode),
        ["usb","ping","--port",path] => output(usb::ping(path)?,json_mode),
        ["service","run"] => crate::service::run()?,
        ["service","status"] => output(json!({"snapshot_available":crate::service::read().is_some()}),json_mode),
        ["data","touch"] => output(crate::projection::compact(&crate::service::snapshot(),true),json_mode),
        ["data","get"] => output(crate::service::snapshot(),json_mode),
        ["cpu","get"] => output(crate::service::snapshot()["cpu"].clone(),json_mode),
        ["ram","get"] => output(crate::service::snapshot()["ram"].clone(),json_mode),
        [module,"get"] if crate::modules::NAMES.contains(module) => output(crate::service::snapshot()[*module].clone(),json_mode),
        ["usb","auto"] => usb::auto_stream()?,
        ["usb","stream","--port",path] => usb::stream(path)?,
        _ => return Err(format!("Unknown command. Run nhc --help")),
    }
    Ok(())
}
