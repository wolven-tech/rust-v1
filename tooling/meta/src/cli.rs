use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "meta")]
#[command(about = "Meta task orchestrator for monorepos", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize meta configuration
    Init,

    /// Start development servers for all projects
    Dev {
        /// Specific projects to run (optional)
        #[arg(short, long)]
        projects: Option<Vec<String>>,
    },

    /// Build projects
    Build {
        /// Production build
        #[arg(long)]
        prod: bool,

        /// Specific projects to build (optional)
        #[arg(short, long)]
        projects: Option<Vec<String>>,
    },

    /// Run tests
    Test {
        /// Watch mode
        #[arg(short, long)]
        watch: bool,
    },

    /// Interactive TUI mode (default)
    Tui,
}
