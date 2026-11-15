mod cli;
mod engine;
mod transport;

fn main() -> std::process::ExitCode {
    match cli::run(std::env::args().skip(1).collect()) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => { eprintln!("nhc: {e}"); std::process::ExitCode::FAILURE }
    }
}

mod modules;

mod deployment;
