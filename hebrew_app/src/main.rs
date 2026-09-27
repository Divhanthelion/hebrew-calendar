//! Hebrew Calendar: a desktop window by default, or a web server with
//! `--server`.
//!
//!   hebrew_app                      # the desktop app
//!   hebrew_app --server             # http://127.0.0.1:3000
//!   hebrew_app --server --host 0.0.0.0 --port 8080

// A release build of the desktop app opens no console window on Windows.
#![cfg_attr(
    all(not(debug_assertions), feature = "gui"),
    windows_subsystem = "windows"
)]

use clap::Parser;

mod config;
mod query;

#[cfg(feature = "server")]
mod api;
#[cfg(feature = "server")]
mod frontend;
#[cfg(feature = "gui")]
mod gui;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A Hebrew calendar with holidays, Torah readings and zmanim"
)]
struct Args {
    /// Run as a web server instead of opening a window.
    #[arg(long, short = 's')]
    server: bool,

    /// Port for the web server.
    #[arg(long, short = 'p', default_value_t = 3000)]
    port: u16,

    /// Address for the web server; 0.0.0.0 to serve other machines.
    #[arg(long, short = 'H', default_value = "127.0.0.1")]
    host: String,
}

fn main() -> anyhow::Result<()> {
    attach_console();
    let args = Args::parse();
    let config = config::AppConfig::load();

    if args.server {
        #[cfg(feature = "server")]
        {
            let runtime = tokio::runtime::Runtime::new()?;
            return runtime.block_on(api::serve(config, &args.host, args.port));
        }
        #[cfg(not(feature = "server"))]
        anyhow::bail!("this build has no web server; build with --features server");
    }

    #[cfg(feature = "gui")]
    {
        gui::launch(config)
    }
    #[cfg(not(feature = "gui"))]
    {
        let _ = config;
        anyhow::bail!("this build has no window; run with --server, or build with --features gui")
    }
}

/// A windowed release build has no console of its own; when started from a
/// terminal (for `--server` or `--help`), write to that terminal.
#[cfg(all(windows, not(debug_assertions), feature = "gui"))]
fn attach_console() {
    use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
    // SAFETY: AttachConsole has no preconditions; failing (no parent console) is harmless.
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[cfg(not(all(windows, not(debug_assertions), feature = "gui")))]
fn attach_console() {}
