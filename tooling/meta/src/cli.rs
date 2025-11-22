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

    /// Run a specific task (e.g., meta run fmt, meta run clippy)
    Run {
        /// Task name to run
        task: String,

        /// Specific projects to run task for (optional)
        #[arg(short, long)]
        projects: Option<Vec<String>>,
    },

    /// Validate meta.toml configuration and check tool availability
    Doctor,
}
