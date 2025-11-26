use std::collections::HashMap;

use anyhow::Result;

use crate::{adapters::ToolAdapter, config::Config};

pub async fn dev(config: &Config, projects: Option<Vec<String>>) -> Result<()> {
    let projects_to_run = get_projects_to_run(config, projects)?;

    let mut commands = Vec::new();

    println!("🚀 Development Commands:\n");

    for (name, project) in &projects_to_run {
        if let Some(dev_task) = project.tasks.get("dev") {
            let tool = config
                .tools
                .get(&dev_task.tool)
                .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", dev_task.tool))?;

            let command_str = dev_task.command.clone();
            let project_path = &project.path;

            // Turborepo runs from workspace root, other tools run from project directory
            let full_command = if tool.command == "turbo" {
                // Turbo runs from workspace root
                format!("{} {}", tool.command, command_str)
            } else {
                // Other tools (bacon, cargo) run from project directory
                format!("cd {} && {} {}", project_path, tool.command, command_str)
            };

            commands.push((name.clone(), full_command.clone()));

            println!("  {} [{}]: {}", name, dev_task.tool, full_command);
        }
    }

    if commands.is_empty() {
        println!("\n⚠️  No dev tasks configured");
        return Ok(());
    }

    println!("\n💡 Launch Options:");
    println!("  1. Manual: Run each command in a separate terminal");
    println!("  2. Tmux: meta will launch all commands in tmux panes (recommended)\n");

    // Auto-launch with tmux if available
    let tmux_available = tokio::process::Command::new("tmux")
        .arg("-V")
        .output()
        .await
        .is_ok();

    if tmux_available && commands.len() > 1 {
        println!(
            "✨ Launching tmux session with {} panes...\n",
            commands.len()
        );
        launch_tmux_session(&commands).await?;
    } else if !tmux_available {
        println!("⚠️  tmux not found. Install tmux to automatically launch all commands.");
        println!("   For now, run these commands manually in separate terminals.");
    } else {
        // Only one command, just run it directly
        println!("Running single command: {}\n", commands[0].1);
        let parts: Vec<&str> = commands[0].1.split("&&").collect();
        if parts.len() == 2 {
            let dir = parts[0].trim().strip_prefix("cd ").unwrap_or(".");
            let cmd_parts: Vec<&str> = parts[1].split_whitespace().collect();
            if !cmd_parts.is_empty() {
                let mut cmd = tokio::process::Command::new(cmd_parts[0]);
                cmd.args(&cmd_parts[1..]);
                cmd.current_dir(dir);
                cmd.stdout(std::process::Stdio::inherit());
                cmd.stderr(std::process::Stdio::inherit());
                cmd.stdin(std::process::Stdio::inherit());
                let status = cmd.status().await?;
                if !status.success() {
                    anyhow::bail!("Command failed");
                }
            }
        }
    }

    Ok(())
}

async fn launch_tmux_session(commands: &[(String, String)]) -> Result<()> {
    use tokio::process::Command;

    let session_name = "meta-dev";

    // Kill existing session if it exists
    let _ = Command::new("tmux")
        .args(["kill-session", "-t", session_name])
        .output()
        .await;

    // Wrap commands in a shell that keeps the pane alive after the command exits
    let wrap_command = |cmd: &str| -> String {
        format!(
            "{}; echo '\\n✓ Process exited. Press Ctrl+C to close or Enter to restart.'; read -r; \
             {}",
            cmd, cmd
        )
    };

    // Create new session with first command
    let first_cmd = &commands[0];
    let wrapped_first = wrap_command(&first_cmd.1);
    Command::new("tmux")
        .args(["new-session", "-d", "-s", session_name, "-n", &first_cmd.0])
        .arg(&wrapped_first)
        .output()
        .await?;

    // Add remaining commands as new panes
    for (name, cmd) in commands.iter().skip(1) {
        let wrapped_cmd = wrap_command(cmd);
        Command::new("tmux")
            .args(["split-window", "-t", session_name, "-h"])
            .arg(&wrapped_cmd)
            .output()
            .await?;

        Command::new("tmux")
            .args(["select-pane", "-t", session_name, "-T", name])
            .output()
            .await?;
    }

    // Tile the panes evenly
    Command::new("tmux")
        .args(["select-layout", "-t", session_name, "tiled"])
        .output()
        .await?;

    // Attach to the session
    println!("📺 Attaching to tmux session '{}'...", session_name);
    println!("\n╭─────────────────────────────────────────────────────────╮");
    println!("│ 🎮 Tmux Navigation Guide                                │");
    println!("├─────────────────────────────────────────────────────────┤");
    println!("│ Navigate Panes:  Ctrl+B then Arrow Keys (← → ↑ ↓)      │");
    println!("│ Zoom Pane:       Ctrl+B then Z (toggle full screen)    │");
    println!("│ Show Numbers:    Ctrl+B then Q (then press number)     │");
    println!("│                                                         │");
    println!("│ Detach Session:  Ctrl+B then D (keeps running)         │");
    println!("│ Stop Process:    Ctrl+C (in current pane)              │");
    println!("│ Close Pane:      Ctrl+B then X (confirm with y)        │");
    println!("╰─────────────────────────────────────────────────────────╯\n");

    let status = Command::new("tmux")
        .args(["attach-session", "-t", session_name])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        anyhow::bail!("Failed to attach to tmux session");
    }

    Ok(())
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
pub async fn doctor(config: &Config) -> Result<()> {
    println!("🏥 Meta Doctor - Configuration Diagnostics\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut errors = 0;
    let mut warnings = 0;

    // Check meta.toml exists and is valid
    println!("📋 Configuration File:");
    println!("  ✓ meta.toml loaded successfully");
    println!("  ✓ Workspace: {}\n", config.workspace.name);

    // Check tools
    println!("🔧 Tool Availability:");
    for (tool_name, tool_config) in &config.tools {
        if !tool_config.enabled {
            println!("  ⊘ {} (disabled)", tool_name);
            continue;
        }

        match tokio::process::Command::new(&tool_config.command)
            .arg("--version")
            .output()
            .await
        {
            Ok(output) if output.status.success() => {
                let version_str = String::from_utf8_lossy(&output.stdout);
                let version = version_str.lines().next().unwrap_or("unknown").trim();
                println!("  ✓ {} → {} ({})", tool_name, tool_config.command, version);
            }
            Ok(_) => {
                println!(
                    "  ⚠ {} → {} (found but version check failed)",
                    tool_name, tool_config.command
                );
                warnings += 1;
            }
            Err(_) => {
                println!("  ✗ {} → {} (NOT FOUND)", tool_name, tool_config.command);
                errors += 1;
            }
        }
    }

    // Check tmux for multi-process support
    println!("\n🖥️  Terminal Multiplexer:");
    match tokio::process::Command::new("tmux")
        .arg("-V")
        .output()
        .await
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("  ✓ tmux → {} (multi-process dev mode available)", version);
        }
        _ => {
            println!("  ⚠ tmux not found (install for 'meta dev' multi-process mode)");
            println!("    Install: brew install tmux (macOS) or apt install tmux (Linux)");
            warnings += 1;
        }
    }

    // Check projects
    println!("\n📦 Projects ({}):", config.projects.len());
    for (name, project) in &config.projects {
        let path = std::path::Path::new(&project.path);
        if path.exists() {
            println!("  ✓ {} → {} ({})", name, project.path, project.project_type);

            // Check if project has dev task
            if project.tasks.contains_key("dev") {
                println!("    • dev task configured");
            }
        } else {
            println!("  ✗ {} → {} (PATH NOT FOUND)", name, project.path);
            errors += 1;
        }
    }

    // Check for common issues
    println!("\n🔍 Configuration Validation:");

    // Validate turborepo commands
    for (name, project) in &config.projects {
        if let Some(dev_task) = project.tasks.get("dev") {
            if dev_task.tool == "turborepo" {
                let cmd = &dev_task.command;
                if !cmd.starts_with("run ") {
                    println!("  ⚠ {} dev task should start with 'run': '{}'", name, cmd);
                    println!("    Suggested: 'run dev --filter=...'");
                    warnings += 1;
                }
                if !cmd.contains("--filter=") {
                    println!("  ⚠ {} turbo task missing --filter flag: '{}'", name, cmd);
                    warnings += 1;
                }
            }
        }
    }

    // Summary
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n📊 Summary:");

    if errors == 0 && warnings == 0 {
        println!("  🎉 All checks passed! Meta is ready to use.");
        println!("\n💡 Quick Start:");
        println!("  • Run 'meta dev' to start all development servers");
        println!("  • Run 'meta dev --projects api' to start specific project");
        println!("  • Run 'meta run <task>' to execute any task across projects");
    } else {
        if errors > 0 {
            println!("  ✗ {} error(s) found - these must be fixed", errors);
        }
        if warnings > 0 {
            println!(
                "  ⚠ {} warning(s) - meta will work but with reduced functionality",
                warnings
            );
        }
    }

    println!();

    if errors > 0 {
        anyhow::bail!("Configuration has errors");
    }

    Ok(())
}
