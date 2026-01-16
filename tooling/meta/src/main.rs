use anyhow::Result;
use clap::Parser;
use tracing::info;

mod adapters;
mod cli;
mod config;
mod execution;

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
        Commands::DevStop => {
            info!("Stopping development servers...");
            execution::dev_stop().await
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
        Commands::Doctor => {
            info!("Running diagnostics...");
            let config = Config::load()?;
            execution::doctor(&config).await
        }
        Commands::Status { project, lines } => {
            let config = Config::load()?;
            execution::status(&config, project, lines).await
        }
        Commands::Logs {
            project,
            follow,
            lines,
        } => {
            let config = Config::load()?;
            execution::logs(&config, project, follow, lines).await
        }
        Commands::Sessions => {
            info!("Listing active meta sessions...");
            execution::sessions().await
        }
    }
}
