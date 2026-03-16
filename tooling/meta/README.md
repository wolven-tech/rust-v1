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
- **Per-Project Logging** - Stdout captured to `.meta/logs/<project>.log` (ANSI-stripped for readability)
- **Project Exclusion** - `dev_default = false` to exclude projects from default `meta dev`
- **Workspace-Aware** - Detects Cargo workspaces for correct binary paths
- **Bacon Validation** - `meta doctor` warns about missing `bacon.toml`
- **Process Tree Detection** - Detects running bacon/cargo processes even when shell wrappers exit
- **Multi-Workspace** - Run meta in multiple directories without conflicts
- **Claude Code Integration** - AI skill for natural language control
- **Zero Config** - Auto-detects projects, generates `meta.toml`

## CLI Reference

| Command | Description |
|---------|-------------|
| `meta dev` | Start all dev servers in tmux |
| `meta dev -p api web` | Start specific projects only |
| `meta dev -d` | Start in background (for CI/agents) |
| `meta dev:stop` | Stop all dev processes |
| `meta status` | Show running processes and logs |
| `meta status --json` | JSON output for programmatic use |
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

# Exclude from default `meta dev` but still usable with `meta dev -p mobile`
[projects.mobile]
type = "rust"
path = "apps/mobile"
dev_default = false

[projects.mobile.tasks]
dev = { tool = "cargo", command = "tauri android dev" }
```

### `dev_default`

Set `dev_default = false` on a project to exclude it from `meta dev` while keeping it available via `meta dev -p <name>`. Useful for projects that require special hardware (emulators, devices) or conflict with other projects on the same port.

## Logging

Meta automatically captures output from all dev processes to `.meta/logs/<project>.log` using tmux's `pipe-pane`. ANSI escape codes are stripped so logs are readable even from TUI tools like bacon.

```bash
meta logs api           # View last 50 lines
meta logs api -l 100    # View last 100 lines
meta logs api -f        # Stream logs in real-time
```

No `bacon.toml` changes are needed — meta handles log capture externally.

## Documentation

- **[User Guide](docs/USER_GUIDE.md)** - Daily workflow, tmux navigation, troubleshooting
- **[Standalone Setup](STANDALONE.md)** - Add meta to your own monorepo
- **[Contributing](CONTRIBUTING.md)** - Development setup and guidelines

## Changelog

### v0.7.1 (Current)
- **Detach mode** — `meta dev -d` / `meta dev --detach` starts services without attaching to tmux ([#8](https://github.com/wolven-tech/rust-v1/issues/8))
- **Non-interactive detection** — Auto-detaches when run from CI, agents, or scripts (no more "failed to attach" errors)
- **JSON status output** — `meta status --json` for programmatic consumption by AI agents and scripts ([#9](https://github.com/wolven-tech/rust-v1/issues/9))
- **Tracing to stderr** — Log output no longer pollutes stdout (clean JSON/pipe output)

### v0.7.0
- **Project exclusion** - `dev_default = false` to exclude projects from default `meta dev` ([#1](https://github.com/wolven-tech/rust-v1/issues/1))
- **Status filtering** - `meta status` only shows projects with a dev task ([#2](https://github.com/wolven-tech/rust-v1/issues/2))
- **Bacon validation** - `meta doctor` warns about missing `bacon.toml` or undefined jobs ([#3](https://github.com/wolven-tech/rust-v1/issues/3))
- **Process tree detection** - Walks full process tree to detect bacon-spawned processes ([#4](https://github.com/wolven-tech/rust-v1/issues/4))
- **Log capture for bacon** - Uses `tmux pipe-pane` with ANSI stripping for all tools including bacon ([#5](https://github.com/wolven-tech/rust-v1/issues/5))
- **Workspace-aware binary paths** - Detects Cargo workspaces and checks correct `target/` directory ([#6](https://github.com/wolven-tech/rust-v1/issues/6))
- **Doctor accuracy** - Per-project dev task check is independent of shared paths ([#7](https://github.com/wolven-tech/rust-v1/issues/7))
- **Library crate skipping** - Binary status checks skip library crates automatically
- **48 tests** - 33 unit + 5 config + 10 integration tests

### v0.6.1
- Bacon logging fix - bacon jobs can now tee output to log files
- Meta skips outer tee for bacon (avoids TUI escape code capture)
- Centralized log location for multi-bacon monorepos

### v0.6.0
- Claude Code skill with decision trees and output parsing
- Multi-instance support (directory-based session names)
- `meta sessions` command
- Fixed process detection via tmux pane queries

<details>
<summary>Earlier versions</summary>

### v0.5.0
- Per-project log capture to `.meta/logs/<project>.log`
- `meta logs` command with `--follow` and `--lines`
- Log rotation at 10MB

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

### v0.8.0 (Next)
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
