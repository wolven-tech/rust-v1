---
name: meta-monorepo
description: Manage polyglot monorepo using the meta CLI. Use for development servers, builds, tests, process status, and orchestrating Turborepo/Cargo/Bacon across projects.
---

# Meta CLI - Polyglot Monorepo Orchestrator

Meta is a unified CLI for managing polyglot monorepos. It orchestrates Turborepo (TypeScript), Cargo (Rust), and Bacon (Rust hot-reload) in tmux sessions.

## Quick Reference

```bash
# Check what's running and detect stale processes
meta status

# Start all dev servers in tmux
meta dev

# Start specific project only
meta dev --projects api

# Stop all dev servers
meta dev:stop

# Run tasks across projects
meta run fmt        # Format all
meta run clippy     # Lint Rust
meta run check      # Type check

# Build and test
meta build
meta test

# Validate setup
meta doctor
```

## Understanding `meta status`

The status command is designed to help detect stale processes:

```
=== META DEV STATUS ===
Log file: .meta/logs/dev.log

## Running Processes
PROJECT         PID        STARTED                      UPTIME
----------------------------------------------------------------------
api             12345      Mon Dec  8 12:02:18 2025     02:30:00
web             -          not running                  -

## Recent Events (last 20)
[2025-12-08T12:02:18] [api] START: Process started (pid=12345)
[2025-12-08T12:05:32] [api] RESTART: Manual restart triggered

## Binary Status (Rust projects)
apps/api/target/debug/api: rebuilt 5m ago ✓ running latest binary
```

### Stale Process Detection

If you see:
```
apps/api/target/debug/api: rebuilt 2m ago ⚠️  STALE: binary rebuilt 10m after process started
```

This means the binary was recompiled but the running process is still the old version. **Restart needed!**

## Development Workflow

### Starting Development

```bash
meta dev              # Launches tmux with all projects
```

This creates a `meta-dev` tmux session with panes for each project:
- Rust projects use `bacon run-long` (auto-restart on changes)
- TypeScript projects use Turborepo

### Tmux Navigation

Once in the tmux session:
- `Ctrl+B` then arrow keys - Navigate between panes
- `Ctrl+B` then `Z` - Zoom current pane (toggle fullscreen)
- `Ctrl+B` then `D` - Detach (keeps running in background)
- `Ctrl+B` then `Q` - Show pane numbers

### Checking Status

```bash
meta status                 # Full status
meta status -p api          # Filter to specific project
meta status -l 50           # Show more log entries
```

### Stopping Development

```bash
meta dev:stop               # Kills the meta-dev tmux session
```

## Configuration

Meta reads from `meta.toml` in the workspace root:

```toml
[workspace]
name = "My Monorepo"
root = "."

[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript", "javascript"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]

[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
test = { tool = "cargo", command = "test" }
```

## Bacon Integration

For Rust projects, bacon handles hot-reload with `kill_then_restart` strategy:

```toml
# apps/api/bacon.toml
[jobs.run-long]
command = ["cargo", "run", "--color", "always"]
on_change_strategy = "kill_then_restart"
watch = ["src", "Cargo.toml", ".env"]
```

When files change:
1. Bacon kills the running cargo process
2. Rebuilds the binary
3. Restarts automatically

**Important**: Use `bacon run-long` (not just `bacon`) to get auto-restart. Running `bacon check` only rebuilds but doesn't restart.

## Log Files

- `.meta/logs/dev.log` - Process start/stop/restart events
- Log format: `[TIMESTAMP] [PROJECT] EVENT: message`

## Troubleshooting

### Process not restarting on changes

1. Check you're using `bacon run-long` not `bacon check`
2. Verify `on_change_strategy = "kill_then_restart"` in bacon.toml
3. Run `meta status` to see if process is stale

### Binary rebuilt but process is stale

```bash
meta status   # Check for ⚠️  STALE warning
meta dev:stop # Stop everything
meta dev      # Restart fresh
```

### tmux session issues

```bash
tmux kill-session -t meta-dev   # Force kill
meta dev                        # Start fresh
```

## Project Types

| Type | Dev Tool | Build Tool |
|------|----------|------------|
| `rust` | bacon | cargo |
| `next` | turborepo | turborepo |
| `typescript` | turborepo | turborepo |

## Best Practices

1. **Always check status first**: `meta status` before assuming something is running
2. **Use stale detection**: The ⚠️ STALE warning means restart is needed
3. **Let bacon handle restarts**: Don't manually restart Rust servers during dev
4. **Detach, don't stop**: Use `Ctrl+B D` to detach and keep servers running
5. **Check logs**: `.meta/logs/dev.log` shows restart history
