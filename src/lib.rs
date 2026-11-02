//! mayu: run commands under a shared per-user CPU and memory budget.
//!
//! Scaffold only so far: the runner itself (daemon, shim, backends) is still to be built.

use std::process::ExitCode;

/// The crate version, as printed by `mayu --version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Entry point shared by the `mayu` and `mayu-run` binaries.
pub fn main() -> ExitCode {
    match std::env::args_os()
        .nth(1)
        .as_deref()
        .and_then(|a| a.to_str())
    {
        Some("--version" | "-V") => {
            println!("mayu {VERSION}");
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("mayu {VERSION}: not implemented yet");
            // 125 is the runner's own-failure exit code, so callers never mistake this for the command's.
            ExitCode::from(125)
        }
    }
}
