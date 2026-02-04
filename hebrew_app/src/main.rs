//! Hebrew Calendar Application
//! 
//! Dual-mode application that can run as either:
//! - A desktop GUI application (Tauri)
//! - An HTTP API server (Axum)
//! 
//! Usage:
//!   hebrew_app              # Run GUI mode (default)
//!   hebrew_app --server     # Run API server mode
//!   hebrew_app --server -p 8080  # Run API server on port 8080

use clap::Parser;
use tracing::info;

mod config;

#[cfg(feature = "server")]
mod api;

#[cfg(feature = "gui")]
mod gui;

/// Hebrew Calendar Application - Dual Mode (GUI / API Server)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in API server mode
    #[arg(long, short = 's')]
    server: bool,
    
    /// Port for API server (only used with --server)
    #[arg(long, short = 'p', default_value_t = 3000)]
    port: u16,
    
    /// Host for API server (only used with --server)
    #[arg(long, short = 'H', default_value = "0.0.0.0")]
    host: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    // Parse CLI arguments
    let args = Args::parse();
    
    // Load configuration
    let config = config::AppConfig::load()?;
    info!("Configuration loaded from {:?}", config::AppConfig::config_path()?);
    
    // Determine mode and launch
    if args.server {
        #[cfg(feature = "server")]
        {
            info!("🚀 Starting in API SERVER mode on {}:{}", args.host, args.port);
            api::launch(config, args.port).await?;
        }
        
        #[cfg(not(feature = "server"))]
        {
            eprintln!("Error: Server mode not available. Compile with --features server");
            std::process::exit(1);
        }
    } else {
        #[cfg(feature = "gui")]
        {
            info!("🖥️  Starting in GUI mode");
            gui::launch(config)?;
        }
        
        #[cfg(not(feature = "gui"))]
        {
            eprintln!("Error: GUI mode not available. Compile with --features gui");
            eprintln!("Hint: Use --server flag to run in API mode");
            std::process::exit(1);
        }
    }
    
    Ok(())
}
