use std::collections::HashMap;

use anyhow::Result;
use tokio::process::Command;

use crate::{adapters::ToolAdapter, config::Config};

/// Show status of running dev processes
///
/// # Output Format (designed for Claude Code parsing)
///
/// ```text
/// === META DEV STATUS ===
/// Log file: .meta/logs/dev.log
///
/// ## Running Processes
/// PROJECT    PID      STARTED                    UPTIME
/// api        12345    2025-12-08T12:02:18        1h 23m
/// web        12346    2025-12-08T12:02:19        1h 23m
///
/// ## Recent Events (last 20)
/// [2025-12-08T12:02:18] [api] START: Process started (pid=12345)
/// [2025-12-08T12:05:32] [api] RESTART: bacon rebuild detected
/// ...
///
/// ## Binary Modification Times
/// apps/api/target/debug/api: 2025-12-08T12:05:30 (rebuilt 1h 20m ago)
/// ```
pub async fn status(config: &Config, project: Option<String>, lines: usize) -> Result<()> {
    println!("=== META DEV STATUS ===");
    println!("Log file: .meta/logs/dev.log\n");

    // Check if tmux session exists
    let session_check = Command::new("tmux")
        .args(["has-session", "-t", "meta-dev"])
        .output()
        .await;

    let session_active = session_check.map(|o| o.status.success()).unwrap_or(false);

    if !session_active {
        println!("⚠️  No active meta-dev session. Run 'meta dev' to start.\n");
    }

    // Show running processes
    println!("## Running Processes");
    println!(
        "{:<15} {:<10} {:<28} UPTIME",
        "PROJECT", "PID", "STARTED"
    );
    println!("{}", "-".repeat(70));

    for (name, proj) in &config.projects {
        if let Some(ref filter) = project {
            if name != filter {
                continue;
            }
        }

        // Find processes related to this project
        // For Rust projects, try to get the actual binary name from Cargo.toml
        let binary_name = if proj.project_type == "rust" {
            get_rust_binary_name(&proj.path).unwrap_or_else(|| name.clone())
        } else {
            name.clone()
        };

        let search_pattern = if proj.project_type == "rust" {
            format!("target/debug/{}", binary_name)
        } else {
            proj.path.clone()
        };

        let ps_output = Command::new("sh")
            .args([
                "-c",
                &format!(
                    "ps aux | grep -E '{}' | grep -v grep | head -1",
                    search_pattern
                ),
            ])
            .output()
            .await;

        if let Ok(output) = ps_output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                // Parse ps output to get PID
                let parts: Vec<&str> = stdout.split_whitespace().collect();
                if parts.len() >= 2 {
                    let pid = parts[1];

                    // Get detailed process info
                    let detail_output = Command::new("ps")
                        .args(["-p", pid, "-o", "pid,lstart,etime"])
                        .output()
                        .await;

                    if let Ok(detail) = detail_output {
                        let detail_str = String::from_utf8_lossy(&detail.stdout);
                        let lines_vec: Vec<&str> = detail_str.lines().collect();
                        if lines_vec.len() >= 2 {
                            let info_parts: Vec<&str> = lines_vec[1].split_whitespace().collect();
                            if info_parts.len() >= 6 {
                                let started = info_parts[1..6].join(" ");
                                let uptime = info_parts.last().unwrap_or(&"?");
                                println!("{:<15} {:<10} {:<28} {}", name, pid, started, uptime);
                            }
                        }
                    }
                }
            } else {
                println!("{:<15} {:<10} {:<28} -", name, "-", "not running");
            }
        }
    }

    // Show recent log events
    println!("\n## Recent Events (last {})", lines);
    let log_path = std::path::Path::new(".meta/logs/dev.log");

    if log_path.exists() {
        let log_content = std::fs::read_to_string(log_path)?;
        let log_lines: Vec<&str> = log_content.lines().collect();

        let filtered_lines: Vec<&str> = if let Some(ref filter) = project {
            log_lines
                .iter()
                .filter(|line| line.contains(&format!("[{}]", filter)))
                .copied()
                .collect()
        } else {
            log_lines
        };

        let start = filtered_lines.len().saturating_sub(lines);
        for line in &filtered_lines[start..] {
            println!("{}", line);
        }

        if filtered_lines.is_empty() {
            println!("(no log entries yet)");
        }
    } else {
        println!("(no log file yet - will be created on next 'meta dev')");
    }

    // Show binary modification times for Rust projects and detect stale processes
    println!("\n## Binary Status (Rust projects)");
    println!(
        "# NOTE: If binary is newer than process, bacon rebuilt but process may be stale.\n# This \
         can happen if bacon is running in check mode instead of run-long mode.\n"
    );

    for (name, proj) in &config.projects {
        if proj.project_type != "rust" {
            continue;
        }
        if let Some(ref filter) = project {
            if name != filter {
                continue;
            }
        }

        // Get actual binary name from Cargo.toml
        let binary_name = get_rust_binary_name(&proj.path).unwrap_or_else(|| name.clone());
        let binary_path = format!("{}/target/debug/{}", proj.path, binary_name);
        let path = std::path::Path::new(&binary_path);

        if path.exists() {
            if let Ok(metadata) = path.metadata() {
                if let Ok(binary_modified) = metadata.modified() {
                    let binary_age = std::time::SystemTime::now()
                        .duration_since(binary_modified)
                        .unwrap_or_default();

                    // Check if there's a running process and compare times
                    let search_pattern = format!("target/debug/{}", binary_name);
                    let ps_output = Command::new("sh")
                        .args([
                            "-c",
                            &format!(
                                "ps aux | grep -E '{}' | grep -v grep | head -1",
                                search_pattern
                            ),
                        ])
                        .output()
                        .await;

                    let mut process_status = String::new();

                    if let Ok(output) = ps_output {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        if !stdout.trim().is_empty() {
                            let parts: Vec<&str> = stdout.split_whitespace().collect();
                            if parts.len() >= 2 {
                                let pid = parts[1];
                                // Get elapsed time using ps -o etime (format: [[DD-]HH:]MM:SS)
                                let etime_output = Command::new("ps")
                                    .args(["-p", pid, "-o", "etime="])
                                    .output()
                                    .await;

                                if let Ok(etime) = etime_output {
                                    let etime_str =
                                        String::from_utf8_lossy(&etime.stdout).trim().to_string();
                                    if let Some(process_age_secs) = parse_etime(&etime_str) {
                                        let binary_age_secs = binary_age.as_secs();

                                        // Only consider stale if binary is more than 60 seconds
                                        // newer than process (to avoid false positives from timing)
                                        let stale_threshold_secs = 60;
                                        if binary_age_secs + stale_threshold_secs < process_age_secs
                                        {
                                            // Binary is significantly newer than process - STALE!
                                            let diff_secs = process_age_secs - binary_age_secs;
                                            let diff_mins = diff_secs / 60;
                                            process_status = format!(
                                                " ⚠️  STALE: binary rebuilt {}m after process \
                                                 started",
                                                diff_mins
                                            );
                                        } else {
                                            process_status = " ✓ running latest binary".to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let mins = binary_age.as_secs() / 60;
                    let hours = mins / 60;
                    let age_str = if hours > 0 {
                        format!("{}h {}m ago", hours, mins % 60)
                    } else {
                        format!("{}m ago", mins)
                    };

                    println!("{}: rebuilt {}{}", binary_path, age_str, process_status);
                }
            }
        } else {
            println!("{}: not built yet", binary_path);
        }
    }

    println!();
    Ok(())
}

/// Get the actual binary name from a Rust project's Cargo.toml
/// Returns None if Cargo.toml doesn't exist or can't be parsed
fn get_rust_binary_name(project_path: &str) -> Option<String> {
    let cargo_path = std::path::Path::new(project_path).join("Cargo.toml");
    let content = std::fs::read_to_string(cargo_path).ok()?;

    // First check for [[bin]] section with name
    // Then fall back to [package] name
    for line in content.lines() {
        let line = line.trim();
        // Look for name = "..." pattern
        if line.starts_with("name") && line.contains('=') {
            if let Some(name_part) = line.split('=').nth(1) {
                let name = name_part.trim().trim_matches('"').trim_matches('\'');
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
        }
    }
    None
}

/// Parse `ps -o etime` format: [[DD-]HH:]MM:SS
/// Examples: "02:30" (2m30s), "01:02:30" (1h2m30s), "2-01:02:30" (2d1h2m30s)
fn parse_etime(s: &str) -> Option<u64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    let mut total_secs: u64 = 0;

    // Check for days (format: DD-HH:MM:SS)
    let (days, rest) = if s.contains('-') {
        let parts: Vec<&str> = s.splitn(2, '-').collect();
        let days: u64 = parts[0].parse().ok()?;
        (days, parts.get(1).copied().unwrap_or(""))
    } else {
        (0, s)
    };

    total_secs += days * 24 * 60 * 60;

    // Parse HH:MM:SS or MM:SS
    let time_parts: Vec<&str> = rest.split(':').collect();
    match time_parts.len() {
        2 => {
            // MM:SS
            let mins: u64 = time_parts[0].parse().ok()?;
            let secs: u64 = time_parts[1].parse().ok()?;
            total_secs += mins * 60 + secs;
        }
        3 => {
            // HH:MM:SS
            let hours: u64 = time_parts[0].parse().ok()?;
            let mins: u64 = time_parts[1].parse().ok()?;
            let secs: u64 = time_parts[2].parse().ok()?;
            total_secs += hours * 60 * 60 + mins * 60 + secs;
        }
        _ => return None,
    }

    Some(total_secs)
}

/// Stop all running meta-dev tmux sessions
pub async fn dev_stop() -> Result<()> {
    let session_name = "meta-dev";

    println!("🛑 Stopping meta development session...\n");

    // Check if session exists
    let list_output = Command::new("tmux")
        .args(["has-session", "-t", session_name])
        .output()
        .await;

    match list_output {
        Ok(output) if output.status.success() => {
            // Session exists, kill it
            let kill_result = Command::new("tmux")
                .args(["kill-session", "-t", session_name])
                .output()
                .await?;

            if kill_result.status.success() {
                println!("✅ Stopped tmux session '{}'", session_name);
                println!("\n💡 All development processes have been terminated.");
            } else {
                let stderr = String::from_utf8_lossy(&kill_result.stderr);
                anyhow::bail!("Failed to kill session: {}", stderr);
            }
        }
        Ok(_) => {
            println!("ℹ️  No active meta-dev session found.");
            println!("\n💡 Use 'meta dev' to start development servers.");
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("⚠️  tmux is not installed.");
                println!("   Install tmux to use the multi-process dev mode.");
            } else {
                anyhow::bail!("Failed to check tmux session: {}", e);
            }
        }
    }

    Ok(())
}

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
    let session_name = "meta-dev";

    // Kill existing session if it exists
    let _ = Command::new("tmux")
        .args(["kill-session", "-t", session_name])
        .output()
        .await;

    // Ensure log directory exists
    let log_dir = std::path::Path::new(".meta/logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }

    // Wrap commands in a shell that:
    // 1. Logs process start/restart events to .meta/logs/dev.log
    // 2. Keeps the pane alive after the command exits
    // 3. Allows manual restart with Enter
    //
    // Log format (designed for easy parsing by Claude Code):
    //   [TIMESTAMP] [PROJECT] EVENT: message (pid=PID)
    //
    // Example:
    //   [2025-12-08T12:02:18] [api] START: Process started (pid=12345)
    //   [2025-12-08T12:05:32] [api] RESTART: Process restarted after file change
    // (pid=12346)   [2025-12-08T12:05:32] [api] EXIT: Process exited with code
    // 0
    let wrap_command = |name: &str, cmd: &str| -> String {
        let log_file = ".meta/logs/dev.log";
        format!(
            r#"LOG_FILE="{log_file}"; \
PROJECT="{name}"; \
log_event() {{ echo "[$(date -u +%Y-%m-%dT%H:%M:%S)] [$PROJECT] $1" >> "$LOG_FILE"; }}; \
run_with_logging() {{ \
  log_event "START: Process started (pid=$$)"; \
  {cmd}; \
  EXIT_CODE=$?; \
  log_event "EXIT: Process exited with code $EXIT_CODE"; \
  return $EXIT_CODE; \
}}; \
run_with_logging; \
while true; do \
  echo '\n✓ Process exited. Press Enter to restart or Ctrl+C to close.'; \
  read -r; \
  log_event "RESTART: Manual restart triggered"; \
  run_with_logging; \
done"#,
            log_file = log_file,
            name = name,
            cmd = cmd
        )
    };

    // Create new session with first command
    let first_cmd = &commands[0];
    let wrapped_first = wrap_command(&first_cmd.0, &first_cmd.1);
    Command::new("tmux")
        .args(["new-session", "-d", "-s", session_name, "-n", &first_cmd.0])
        .arg(&wrapped_first)
        .output()
        .await?;

    // Add remaining commands as new panes
    for (name, cmd) in commands.iter().skip(1) {
        let wrapped_cmd = wrap_command(name, cmd);
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
