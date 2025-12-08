# Meta CLI Command Reference

## Installation

```bash
cargo install monorepo-meta
```

Or build from source:
```bash
cd tooling/meta && cargo build --release
```

Binary location: `tooling/meta/target/release/meta`

## Commands

### meta init

Initialize a new meta.toml configuration.

```bash
meta init
```

Auto-detects:
- Rust projects (Cargo.toml)
- Next.js projects (next.config.*)
- Node.js projects (package.json)

### meta dev

Start development servers for all projects in tmux.

```bash
meta dev                        # All projects
meta dev --projects api         # Specific project
meta dev -p api -p web          # Multiple projects
```

**Behavior**:
- Creates `meta-dev` tmux session
- Each project gets its own pane
- Rust projects use bacon with auto-restart
- TypeScript projects use Turborepo

### meta dev:stop

Stop all running development servers.

```bash
meta dev:stop
```

Kills the `meta-dev` tmux session and all child processes.

### meta status

Show status of running dev processes.

```bash
meta status                     # Full status
meta status -p api              # Filter by project
meta status --project api       # Same as above
meta status -l 50               # Show 50 log lines (default: 20)
meta status --lines 100         # Show 100 log lines
```

**Output sections**:
1. Running Processes - PID, start time, uptime
2. Recent Events - From .meta/logs/dev.log
3. Binary Status - Stale process detection for Rust projects

### meta build

Build all projects.

```bash
meta build                      # All projects
meta build --projects api       # Specific project
meta build --prod               # Production build
```

### meta test

Run tests across projects.

```bash
meta test                       # All projects
meta test --watch               # Watch mode (if supported)
```

### meta run

Run arbitrary tasks defined in meta.toml.

```bash
meta run fmt                    # Run 'fmt' task
meta run clippy                 # Run 'clippy' task
meta run check                  # Run 'check' task
meta run audit                  # Run 'audit' task
meta run fmt -p api             # Run for specific project
```

### meta doctor

Validate configuration and check tool availability.

```bash
meta doctor
```

**Checks**:
- meta.toml syntax and validity
- Tool availability (cargo, bacon, turbo, tmux)
- Project paths exist
- Task configurations are valid

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Configuration error |
| 2 | Command not found |
| 101 | Runtime error |

## Environment Variables

Meta respects standard tool environment variables:
- `RUST_LOG` - Logging level for Rust projects
- `NODE_ENV` - Environment for Node.js projects

## Files

| File | Purpose |
|------|---------|
| `meta.toml` | Main configuration (workspace root) |
| `.meta/logs/dev.log` | Process event log |
| `bacon.toml` | Bacon config (per Rust project) |

## Log Format

Events in `.meta/logs/dev.log`:

```
[2025-12-08T12:02:18] [api] START: Process started (pid=12345)
[2025-12-08T12:05:32] [api] EXIT: Process exited with code 0
[2025-12-08T12:05:33] [api] RESTART: Manual restart triggered
```

**Event types**:
- `START` - Process started
- `EXIT` - Process exited (includes exit code)
- `RESTART` - Manual restart (user pressed Enter)
