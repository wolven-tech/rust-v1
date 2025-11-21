use anyhow::Result;
use clap::Parser;
use tracing::info;

mod adapters;
mod cli;
mod config;
mod execution;
mod tui;

use cli::{Cli, Commands};
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt().with_target(false).init();

    let cli = Cli::parse();

    info!("Meta orchestrator starting...");

    match cli.command {
        Commands::Init => {
            info!("Initializing meta configuration...");
            config::init()?;
            println!("✅ Created meta.toml configuration file");
            Ok(())
        }
        Commands::Dev { projects } => {
            info!("Starting development servers...");
            let config = Config::load()?;
            execution::dev(&config, projects).await
        }
        Commands::Build { prod, projects } => {
            info!("Building projects...");
            let config = Config::load()?;
            execution::build(&config, prod, projects).await
        }
        Commands::Test { watch } => {
            info!("Running tests...");
            let config = Config::load()?;
            execution::test(&config, watch).await
        }
        Commands::Run { task, projects } => {
            info!("Running task: {}", task);
            let config = Config::load()?;
            execution::run_task(&config, &task, projects).await
        }
        Commands::Tui => {
            info!("Launching TUI with live log streaming...");
            let config = Config::load()?;

            // Start dev servers with streaming
            let (log_rx, status_rx, process_rx) =
                execution::dev_with_streaming(&config, None).await?;

            // Run TUI with log, status, and process receivers
            tui::run_tui_with_streaming(config, log_rx, status_rx, process_rx).await
        }
    }
}
