mod cli;
mod engine;
mod transport;

fn main() -> std::process::ExitCode {
    match cli::run(std::env::args().skip(1).collect()) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => { eprintln!("nhc: {e}"); std::process::ExitCode::FAILURE }
    }
}

#[cfg(not(windows))]
mod modules;
#[cfg(windows)]
#[path = "windows/mod.rs"]
mod modules;

#[cfg(not(windows))]
mod deployment;
#[cfg(windows)]
#[path = "windows/deployment.rs"]
mod deployment;

mod service;
mod projection;
