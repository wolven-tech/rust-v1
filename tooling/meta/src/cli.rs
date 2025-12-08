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

    /// Stop all running tmux development sessions
    #[command(name = "dev:stop")]
    DevStop,

    /// Show status of running dev processes (useful for Claude Code)
    ///
    /// Displays process info, restart history, and binary modification times.
    /// Log file: .meta/logs/dev.log
    Status {
        /// Show only entries for specific project
        #[arg(short, long)]
        project: Option<String>,

        /// Number of recent log entries to show (default: 20)
        #[arg(short, long, default_value = "20")]
        lines: usize,
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
