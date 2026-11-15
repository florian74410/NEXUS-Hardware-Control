use serde_json::{json, Value};

pub trait Module {
    fn name(&self) -> &'static str;
    fn status(&self) -> Result<Value, String>;
}

pub fn status(modules: &[&dyn Module]) -> Value {
    json!({
        "version": env!("CARGO_PKG_VERSION"),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "mode": "foreground-diagnostics",
        "modules": modules.iter().map(|m| json!({
            "name":m.name(), "status": match m.status() {
                Ok(v) => v, Err(e) => json!({"error":e})
            }
        })).collect::<Vec<_>>()
    })
}
