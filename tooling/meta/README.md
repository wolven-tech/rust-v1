# Meta - Monorepo Task Orchestrator

[![Crates.io](https://img.shields.io/crates/v/monorepo-meta.svg)](https://crates.io/crates/monorepo-meta)
[![Documentation](https://docs.rs/monorepo-meta/badge.svg)](https://docs.rs/monorepo-meta)

> One command to rule them all

**Meta** orchestrates Turborepo, Cargo, and Bacon in tmux for polyglot monorepos. Stop juggling multiple terminals - let meta manage your dev environment.

```bash
cargo install monorepo-meta
```

[![asciicast](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq.svg)](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq)

## Why Meta?

Modern monorepos use **multiple tools**: Turborepo for TypeScript, Cargo for Rust, Bacon for hot-reload. Meta unifies them under one CLI with proper tmux orchestration.

## Quick Start

```bash
# 1. Validate setup
meta doctor

# 2. Start all dev servers in tmux
meta dev

# 3. Navigate: Ctrl+B then arrows | Detach: Ctrl+B then D
```

## Features

- **Tmux Orchestration** - Each project in its own pane with full TUI
- **Smart Routing** - Turborepo from root, Bacon/Cargo from project directory
- **Per-Project Logging** - Stdout captured to `.meta/logs/<project>.log`
- **Multi-Workspace** - Run meta in multiple directories without conflicts
- **Claude Code Integration** - AI skill for natural language control
- **Zero Config** - Auto-detects projects, generates `meta.toml`

## CLI Reference

| Command | Description |
|---------|-------------|
| `meta dev` | Start all dev servers in tmux |
| `meta dev -p api web` | Start specific projects only |
| `meta dev:stop` | Stop all dev processes |
| `meta status` | Show running processes and logs |
| `meta logs <project>` | View project logs (`-f` to follow) |
| `meta sessions` | List all active meta sessions |
| `meta build [--prod]` | Build all projects |
| `meta test` | Run all tests |
| `meta run <task>` | Run any task (fmt, clippy, audit) |
| `meta doctor` | Validate configuration |
| `meta init` | Generate meta.toml |

## Configuration

```toml
# meta.toml
version = "1"

[workspace]
name = "My Monorepo"

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]

[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript"]

[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }

[projects.web]
type = "next"
path = "apps/web"

[projects.web.tasks]
dev = { tool = "turborepo", command = "run dev --filter=@org/web" }
```

## Bacon Logging

For bacon projects to write logs, configure your `bacon.toml` to tee output:

```toml
[jobs.run-long]
# Tee output to centralized log file
command = ["sh", "-c", "mkdir -p ../../.meta/logs && cargo run --color always 2>&1 | tee ../../.meta/logs/api.log"]
need_stdout = true

# Export errors/warnings to centralized location
[exports.meta-locations]
auto = true
path = "../../.meta/logs/api-locations"
line_format = "{kind}|{path}:{line}:{column}|{message}"
```

## Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Daily workflow, tmux navigation, troubleshooting
- **[Standalone Setup](STANDALONE.md)** - Add meta to your own monorepo
- **[Contributing](CONTRIBUTING.md)** - Development setup and guidelines

## Changelog

### v0.6.1 (Current)
- Bacon logging fix - bacon jobs can now tee output to log files
- Meta skips outer tee for bacon (avoids TUI escape code capture)
- Centralized log location for multi-bacon monorepos

### v0.6.0
- Claude Code skill with decision trees and output parsing
- Multi-instance support (directory-based session names)
- `meta sessions` command
- Fixed process detection via tmux pane queries

### v0.5.0
- Per-project log capture to `.meta/logs/<project>.log`
- `meta logs` command with `--follow` and `--lines`
- Log rotation at 10MB

<details>
<summary>Earlier versions</summary>

### v0.4.1
- `meta status` command for process monitoring
- Lifecycle logging (START/EXIT/RESTART events)
- Binary staleness detection

### v0.3.1
- `meta dev:stop` command
- Improved tmux navigation guide

### v0.3.0
- Published to crates.io
- Custom pane titles

### v0.2.1
- Tmux orchestration
- Multiple bacon instances with full TUI
- `meta doctor` validation

</details>

## Roadmap

### v0.7.0 (Next)
- Session save/restore
- Environment support (dev, staging, prod)
- Project dependency ordering

### Future
- Remote execution (SSH)
- Custom tool plugins
- CI/CD integration

## Development

```bash
cd tooling/meta
cargo test              # Run tests
cargo clippy            # Lint
cargo build --release   # Build
```

## License

MIT

---

**Built with Rust**
