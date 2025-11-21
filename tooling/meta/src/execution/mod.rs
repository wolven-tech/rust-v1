use std::collections::HashMap;

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    sync::mpsc,
    task::JoinSet,
};
use tracing::{error, info};

use crate::{adapters::ToolAdapter, config::Config};

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub project: String,
    pub message: String,
    pub timestamp: String,
    pub level: LogLevel,
}

#[derive(Clone, Debug)]
pub struct StatusUpdate {
    pub project: String,
    pub status: ProcessStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ProcessStatus {
    Starting,
    Running,
    Success,
    Failed,
    Crashed,
}

#[derive(Clone, Debug)]
pub enum LogLevel {
    Info,
    Error,
    Debug,
}

pub type LogReceiver = mpsc::UnboundedReceiver<LogMessage>;
pub type StatusReceiver = mpsc::UnboundedReceiver<StatusUpdate>;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub project: String,
    pub pid: u32,
}

pub type ProcessReceiver = mpsc::UnboundedReceiver<ProcessInfo>;

pub async fn dev(config: &Config, projects: Option<Vec<String>>) -> Result<()> {
    let projects_to_run = get_projects_to_run(config, projects)?;

    println!("🚀 Starting development servers...\n");

    let mut join_set = JoinSet::new();

    for (name, project) in projects_to_run {
        if let Some(dev_task) = project.tasks.get("dev") {
            let tool = config
                .tools
                .get(&dev_task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", dev_task.tool))?;

            let adapter = ToolAdapter::new(dev_task.tool.clone(), tool.command.clone());
            let command_str = dev_task.command.clone();
            let project_name = name.clone();
            let project_path = project.path.clone();

            println!("  → {} ({})", project_name, dev_task.tool);

            join_set.spawn(async move {
                info!("Starting {} with command: {}", project_name, command_str);

                // Parse command into args
                let parts: Vec<&str> = command_str.split_whitespace().collect();

                // Use project path for execution
                let path = std::path::Path::new(&project_path);
                match adapter.execute_in(&parts, path).await {
                    Ok(_) => {
                        info!("{} completed successfully", project_name);
                        Ok(())
                    }
                    Err(e) => {
                        error!("{} failed: {}", project_name, e);
                        Err(e)
                    }
                }
            });
        }
    }

    println!("\n⏳ Running... (press Ctrl+C to stop)\n");

    // Wait for all tasks
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(_)) => {}
            Ok(Err(e)) => error!("Task error: {}", e),
            Err(e) => error!("Join error: {}", e),
        }
    }

    Ok(())
}

pub async fn dev_with_streaming(
    config: &Config,
    projects: Option<Vec<String>>,
) -> Result<(LogReceiver, StatusReceiver, ProcessReceiver)> {
    let projects_to_run = get_projects_to_run(config, projects)?;
    let (log_tx, log_rx) = mpsc::unbounded_channel();
    let (status_tx, status_rx) = mpsc::unbounded_channel();
    let (process_tx, process_rx) = mpsc::unbounded_channel();

    let mut join_set = JoinSet::new();

    for (name, project) in projects_to_run {
        if let Some(dev_task) = project.tasks.get("dev") {
            let tool = config
                .tools
                .get(&dev_task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", dev_task.tool))?;

            let adapter = ToolAdapter::new(dev_task.tool.clone(), tool.command.clone());
            let command_str = dev_task.command.clone();
            let project_name = name.clone();
            let project_path = project.path.clone();
            let log_tx_clone = log_tx.clone();
            let status_tx_clone = status_tx.clone();
            let process_tx_clone = process_tx.clone();

            join_set.spawn(async move {
                info!("Starting {} with command: {}", project_name, command_str);

                // Send starting status
                let _ = status_tx_clone.send(StatusUpdate {
                    project: project_name.clone(),
                    status: ProcessStatus::Starting,
                });

                // Parse command into args
                let parts: Vec<&str> = command_str.split_whitespace().collect();
                let parts_owned: Vec<String> = parts.iter().map(|s| s.to_string()).collect();

                // Spawn process and capture output with working directory
                let path = std::path::Path::new(&project_path);
                match adapter.spawn_in(
                    &parts_owned.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
                    path,
                ) {
                    Ok(mut child) => {
                        // Send process ID for cleanup
                        if let Some(pid) = child.id() {
                            let _ = process_tx_clone.send(ProcessInfo {
                                project: project_name.clone(),
                                pid,
                            });
                        }

                        // Send running status
                        let _ = status_tx_clone.send(StatusUpdate {
                            project: project_name.clone(),
                            status: ProcessStatus::Running,
                        });
                        // Capture stdout
                        if let Some(stdout) = child.stdout.take() {
                            let project = project_name.clone();
                            let tx = log_tx_clone.clone();
                            tokio::spawn(async move {
                                let reader = BufReader::new(stdout);
                                let mut lines = reader.lines();
                                while let Ok(Some(line)) = lines.next_line().await {
                                    let level = detect_log_level(&line);
                                    let _ = tx.send(LogMessage {
                                        project: project.clone(),
                                        message: line,
                                        timestamp: chrono::Utc::now()
                                            .format("%H:%M:%S")
                                            .to_string(),
                                        level,
                                    });
                                }
                            });
                        }

                        // Capture stderr
                        if let Some(stderr) = child.stderr.take() {
                            let project = project_name.clone();
                            let tx = log_tx_clone.clone();
                            tokio::spawn(async move {
                                let reader = BufReader::new(stderr);
                                let mut lines = reader.lines();
                                while let Ok(Some(line)) = lines.next_line().await {
                                    let _ = tx.send(LogMessage {
                                        project: project.clone(),
                                        message: line,
                                        timestamp: chrono::Utc::now()
                                            .format("%H:%M:%S")
                                            .to_string(),
                                        level: LogLevel::Error,
                                    });
                                }
                            });
                        }

                        // Wait for process to complete
                        match child.wait().await {
                            Ok(exit_status) => {
                                if exit_status.success() {
                                    info!("{} completed successfully", project_name);
                                    let _ = status_tx_clone.send(StatusUpdate {
                                        project: project_name.clone(),
                                        status: ProcessStatus::Success,
                                    });
                                } else {
                                    error!("{} exited with status: {}", project_name, exit_status);
                                    let _ = status_tx_clone.send(StatusUpdate {
                                        project: project_name.clone(),
                                        status: ProcessStatus::Failed,
                                    });
                                }
                            }
                            Err(e) => {
                                error!("{} wait error: {}", project_name, e);
                                let _ = status_tx_clone.send(StatusUpdate {
                                    project: project_name.clone(),
                                    status: ProcessStatus::Crashed,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to spawn {}: {}", project_name, e);
                        let _ = status_tx_clone.send(StatusUpdate {
                            project: project_name.clone(),
                            status: ProcessStatus::Failed,
                        });
                    }
                }

                Ok::<(), anyhow::Error>(())
            });
        }
    }

    // Spawn a task to wait for all processes
    tokio::spawn(async move {
        while let Some(_result) = join_set.join_next().await {
            // Process results if needed
        }
    });

    Ok((log_rx, status_rx, process_rx))
}

pub async fn build(config: &Config, _prod: bool, projects: Option<Vec<String>>) -> Result<()> {
    let projects_to_build = get_projects_to_run(config, projects)?;

    println!("🔨 Building projects...\n");

    for (name, project) in projects_to_build {
        if let Some(build_task) = project.tasks.get("build") {
            let tool = config
                .tools
                .get(&build_task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", build_task.tool))?;

            let adapter = ToolAdapter::new(build_task.tool.clone(), tool.command.clone());

            println!("  → Building {} ({})", name, build_task.tool);

            let parts: Vec<&str> = build_task.command.split_whitespace().collect();

            // Use project path for execution
            let project_path = std::path::Path::new(&project.path);
            adapter.execute_in(&parts, project_path).await?;
        }
    }

    println!("\n✅ Build complete!\n");
    Ok(())
}

pub async fn test(config: &Config, _watch: bool) -> Result<()> {
    println!("🧪 Running tests...\n");

    for (name, project) in &config.projects {
        if let Some(test_task) = project.tasks.get("test") {
            let tool = config
                .tools
                .get(&test_task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", test_task.tool))?;

            let adapter = ToolAdapter::new(test_task.tool.clone(), tool.command.clone());

            println!("  → Testing {} ({})", name, test_task.tool);

            let parts: Vec<&str> = test_task.command.split_whitespace().collect();

            // Use project path for execution
            let project_path = std::path::Path::new(&project.path);
            adapter.execute_in(&parts, project_path).await?;
        }
    }

    println!("\n✅ Tests complete!\n");
    Ok(())
}

pub async fn run_task(
    config: &Config,
    task_name: &str,
    projects: Option<Vec<String>>,
) -> Result<()> {
    let projects_to_run = get_projects_to_run(config, projects)?;

    println!("🚀 Running task '{}'...\n", task_name);

    for (name, project) in projects_to_run {
        if let Some(task) = project.tasks.get(task_name) {
            let tool = config
                .tools
                .get(&task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", task.tool))?;

            let adapter = ToolAdapter::new(task.tool.clone(), tool.command.clone());

            println!("  → {} ({})", name, task.tool);

            let parts: Vec<&str> = task.command.split_whitespace().collect();

            // Use project path for execution
            let project_path = std::path::Path::new(&project.path);
            adapter.execute_in(&parts, project_path).await?;
        } else {
            println!("  ⊘ {} (task '{}' not defined, skipping)", name, task_name);
        }
    }

    println!("\n✅ Task '{}' complete!\n", task_name);
    Ok(())
}

fn get_projects_to_run(
    config: &Config,
    projects: Option<Vec<String>>,
) -> Result<HashMap<String, &crate::config::ProjectConfig>> {
    let mut result = HashMap::new();

    match projects {
        Some(names) => {
            for name in names {
                let project = config
                    .projects
                    .get(&name)
                    .ok_or_else(|| anyhow::anyhow!("Project not found: {}", name))?;
                result.insert(name, project);
            }
        }
        None => {
            for (name, project) in &config.projects {
                result.insert(name.clone(), project);
            }
        }
    }

    Ok(result)
}

/// Detect log level based on message content with enhanced pattern matching
fn detect_log_level(message: &str) -> LogLevel {
    let lower = message.to_lowercase();

    // Check for error patterns - prioritize these
    if lower.contains("error")
        || lower.contains("fail")
        || lower.contains("panic")
        || lower.contains("fatal")
        || lower.contains("✗")
        || lower.contains("❌")
        || lower.contains("exception")
        || lower.contains("crashed")
        || lower.contains("segfault")
        || lower.contains("abort")
    {
        return LogLevel::Error;
    }

    // Check for warning patterns
    if lower.contains("warn") || lower.contains("warning") || lower.contains("⚠") {
        return LogLevel::Error; // Treat warnings as errors for visibility
    }

    // Check for debug/trace patterns
    if lower.contains("debug") || lower.contains("trace") || lower.contains("verbose") {
        return LogLevel::Debug;
    }

    // Default to Info
    LogLevel::Info
}
