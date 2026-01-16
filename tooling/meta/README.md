# Meta - Monorepo Task Orchestrator

[![Crates.io](https://img.shields.io/crates/v/monorepo-meta.svg)](https://crates.io/crates/monorepo-meta)
[![Documentation](https://docs.rs/monorepo-meta/badge.svg)](https://docs.rs/monorepo-meta)

> One command to rule them all 🚀

**Meta** is a unified task orchestrator for modern monorepos. Stop juggling Turborepo, cargo, and bacon commands - let meta orchestrate multiple bacon instances and turborepo tasks in tmux automatically.

📦 **Install from crates.io:** `cargo install monorepo-meta`

[![asciicast](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq.svg)](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq)

## Why Meta?

Modern monorepos use **multiple specialized tools**:
- 🟦 **Turborepo** - TypeScript/Next.js (with Vercel remote caching)
- 🦀 **Cargo** - Rust builds and tests
- 🥓 **Bacon** - Rust hot-reload development
- 🌙 **Moon** - Polyglot task running (optional)

**The Problem:** Context switching, different CLI syntax, no unified view.

**The Solution:** Meta orchestrates all tools under one interface.

## Features

- ✅ **Smart Routing** - Automatically selects the right tool for each project
- ✅ **Unified CLI** - One command for all tasks (`meta dev`, `meta build`, `meta test`)
- ✅ **Multiple Bacon TUIs** - Each Rust project gets full interactive bacon TUI in its own pane
- ✅ **Tmux Orchestration** - All processes in separate panes with native terminal access
- ✅ **Turborepo Integration** - Workspace-aware task execution from root directory
- ✅ **Zero Config** - Works with sensible defaults, configurable via `meta.toml`
- ✅ **Built-in Validation** - `meta doctor` checks your entire setup
- ✅ **Detach/Reattach** - Keep dev servers running in background
- ✅ **Stop All Sessions** - `meta dev:stop` kills all dev processes instantly
- ✅ **Hot Reload Built-in** - Bacon and Turbo handle file watching natively
- ✅ **Custom Pane Titles** - Each tmux pane shows project name
- ✅ **Per-Project Logging** - Captures stdout/stderr to `.meta/logs/<project>.log`
- ✅ **Log Viewing** - `meta logs` command with follow mode and line limits
- ✅ **Multi-Instance Support** - Run meta in multiple workspaces simultaneously
- ✅ **Session Management** - `meta sessions` lists all active meta sessions
- ✅ **Claude Code Integration** - AI-optimized skill with decision trees and output parsing

## Claude Code Integration

Meta includes a purpose-built **Claude Code skill** (`.claude/skills/meta/SKILL.md`) that enables AI agents to effectively manage your development environment.

### AI Skill Features

| Feature | Description |
|---------|-------------|
| **Decision Tree** | Guides AI to select the correct command for any task |
| **Output Parsing** | Structured patterns for extracting process status, errors, PIDs |
| **Error Taxonomy** | Exit codes mapped to recovery actions |
| **Workflow Templates** | Common multi-step operations (debug, restart, validate) |

### Example: AI Debugging a Crashed Process

When you tell Claude Code "the API server crashed", the skill guides it through:

```bash
# 1. Check what's running
meta status
# AI parses: PROJECT=api, PID=-, STATUS="not running"

# 2. View recent logs for errors
meta logs api -l 100
# AI parses for: error:, panic, FAILED patterns

# 3. Restart the environment
meta dev:stop && meta dev
```

### Example: AI Managing Multiple Workspaces

```bash
# List all active sessions across workspaces
meta sessions

# Output (AI-parseable):
## Active Meta Sessions

  meta-backend (this workspace)
    Panes: 3
  meta-frontend
    Panes: 2
```

### Invoking the Skill

In Claude Code, use the `/meta` command or ask naturally:

```
> /meta status
> "Check if my dev servers are running"
> "Why isn't the API responding?"
> "Start the development environment"
```

The skill file location: `.claude/skills/meta/SKILL.md`

## Installation

```bash
# Install from crates.io (recommended)
cargo install monorepo-meta

# Or install from source
cd tooling/meta && cargo install --path .
```

For detailed setup in your own monorepo, see the **[Standalone Installation Guide](./STANDALONE.md)**.

## Quick Start

> **📖 New to Meta?** See the complete [User Guide](docs/USER_GUIDE.md) for detailed instructions, tmux navigation tips, and recording demos.

### 1. Verify Installation

Check that meta and your configuration are correct:

```bash
meta doctor
```

This will validate:
- ✅ All tools (bacon, cargo, turbo, tmux) are installed
- ✅ meta.toml configuration is valid
- ✅ All project paths exist
- ✅ Tasks are properly configured

### 2. Start Development

**Run all dev servers with tmux:**
```bash
meta dev
```

This launches a tmux session with separate panes for each project:
- Each bacon instance runs with full interactive TUI
- Each turbo process runs from workspace root
- Press `Ctrl+B` then `D` to detach
- Press `Ctrl+C` in each pane to stop

**Run specific projects only:**
```bash
meta dev -p api
meta dev -p web app
```

### 3. Run Tasks

```bash
# Run any task across all projects
meta run fmt
meta run clippy
meta run test

# Run on specific projects
meta run test -p api
```

## How It Works

### Tmux Integration

Meta uses **tmux** to run multiple bacon and turbo instances concurrently:

1. **Detects tmux** - Automatically checks if tmux is installed
2. **Creates session** - Launches `meta-<workspace>` tmux session (unique per directory)
3. **Separate panes** - Each project gets its own pane:
   - Bacon projects: `cd apps/api && bacon run-long`
   - Turbo projects: `turbo run dev --filter=@package/name`
4. **Full TUI** - Each bacon instance runs with complete interactivity
5. **Easy navigation** - Use tmux keys to switch between panes
6. **Multi-instance** - Run different workspaces without conflicts (`meta sessions` to list)

### Tool Routing

Meta automatically routes commands based on project type:

- **Turborepo** → Runs from workspace root with `--filter`
- **Bacon/Cargo** → Changes to project directory first
- **Other tools** → Configurable in meta.toml

### Updating Meta

```bash
# From crates.io
cargo install monorepo-meta

# Check version
meta --version
```

## Configuration

Meta uses `meta.toml` for configuration:

```toml
[workspace]
name = "My Monorepo"
root = "."

[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript"]
for_tasks = ["dev", "build"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]
for_tasks = ["dev"]

[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
```

## CLI Commands

### `meta init`
Initialize `meta.toml` configuration file.

### `meta dev`
Start development servers for all projects.

**Options:**
- `-p, --projects <NAMES>` - Run specific projects only

**Example:**
```bash
# Run all projects
meta dev

# Run only API and web
meta dev -p api -p web
```

### `meta dev:stop`
Stop all running tmux development sessions.

**Example:**
```bash
# Stop all dev servers
meta dev:stop
```

This will kill the `meta-dev` tmux session and all processes running within it.

### `meta build`
Build all projects.

**Options:**
- `--prod` - Production build
- `-p, --projects <NAMES>` - Build specific projects

**Example:**
```bash
# Development build
meta build

# Production build
meta build --prod

# Build specific projects
meta build -p api --prod
```

### `meta test`
Run tests for all projects.

**Options:**
- `-w, --watch` - Watch mode (coming soon)

### `meta run <task>`
Run a specific task across all projects (or selected projects).

**Options:**
- `-p, --projects <NAMES>` - Run task for specific projects only

**Examples:**
```bash
# Run formatting check across all projects
meta run fmt

# Run clippy only on the meta project itself
meta run clippy -p meta

# Run security audit
meta run audit

# Auto-fix formatting
meta run fmt-fix
```

**Common Tasks:**
- `fmt` - Check code formatting
- `fmt-fix` - Auto-format code
- `clippy` - Lint code with zero warnings
- `clippy-fix` - Auto-fix lint issues
- `audit` - Security vulnerability audit
- `check` - Fast compile check without building

### `meta status`
Show status of running dev processes and available logs.

**Options:**
- `-p, --project <NAME>` - Show only entries for specific project
- `-l, --lines <N>` - Number of recent log entries (default: 20)

**Example:**
```bash
meta status

# Output shows:
# ✓ Running processes with PIDs and uptime
# ✓ Recent lifecycle events (START/EXIT/RESTART)
# ✓ Binary rebuild status (detects stale processes)
# ✓ Available project logs with sizes
```

### `meta logs`
View project logs (stdout/stderr captured from dev processes).

**Options:**
- `-f, --follow` - Follow log output in real-time (like `tail -f`)
- `-l, --lines <N>` - Number of lines to show (default: 50)

**Examples:**
```bash
# List available logs
meta logs

# View last 50 lines of api logs
meta logs api

# Follow api logs in real-time
meta logs api --follow

# View last 100 lines
meta logs api -l 100
```

**Log files:**
- Located in `.meta/logs/<project>.log`
- Auto-rotated at 10MB (keeps one backup as `.log.1`)
- Created when running `meta dev`

### `meta sessions`
List all active meta tmux sessions across different workspaces.

**Example:**
```bash
meta sessions

# Output shows:
# ## Active Meta Sessions
#
#   meta-workspace-a (this workspace)
#     Panes: 3
#   meta-workspace-b
#     Panes: 2
```

This is helpful when running meta in multiple workspaces simultaneously. Each workspace gets its own uniquely named session based on the directory name.

### `meta doctor`
Validate your entire setup before starting development.

**Checks:**
- All required tools are installed (bacon, cargo, turbo, tmux)
- meta.toml configuration is valid
- All project paths exist
- Tasks are properly configured
- Turborepo command syntax is correct

**Example:**
```bash
meta doctor

# Output shows:
# ✓ Tool availability with versions
# ✓ Project validation
# ✓ Configuration validation
# ✓ Quick start suggestions
```

## Demo

See Meta in action:

[![asciicast](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq.svg)](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq)

**Demo showcases:**
- `meta doctor` - Configuration validation
- `meta dev` - Tmux session launch with multiple bacon instances
- Tmux navigation - Full interactive bacon TUI per project
- Turborepo integration - Workspace-aware execution
- Detach/reattach - Keep dev servers running in background

## Recording Your Own Demos

Want to create a demo of Meta for documentation or social media?

**Quick start:**
```bash
cd ../../docs/launch  # From meta directory
./record-demo.sh      # Record with asciinema
./convert-to-gif.sh   # Convert to GIF
```

See [docs/launch/DEMO_SCRIPT.md](../../docs/launch/DEMO_SCRIPT.md) for detailed recording instructions.

## Architecture

```
meta/
├── src/
│   ├── cli.rs         # CLI parsing (clap)
│   ├── config.rs      # meta.toml loading
│   ├── adapters/      # Tool adapters
│   └── execution/     # Task execution & tmux orchestration
├── Cargo.toml
└── README.md
```

## Comparison

| Tool | Purpose | Meta Advantage |
|------|---------|----------------|
| **mprocs/overmind** | Process orchestrator | Native TTY per process, bacon TUI support, tool-aware routing |
| **Turborepo** | JS/TS monorepo | Multi-language support (Rust + TS), proper bacon integration |
| **Just/Taskfile** | Task runner | Tmux orchestration, detach/reattach, validation |

## Roadmap

### v0.1.0 ✅
- [x] CLI with basic commands
- [x] Configuration loading
- [x] Tool adapters (turbo, cargo, bacon)
- [x] Task routing
- [x] Parallel execution

### v0.2.1 ✅
- [x] Tmux orchestration
- [x] Multiple bacon instances with full TUI
- [x] Turborepo workspace integration
- [x] Built-in validation (doctor command)
- [x] Detach/reattach support
- [x] Tool-aware routing

### v0.3.0 ✅
- [x] Custom pane titles (each tmux pane shows project name)
- [x] Clean documentation (no bun wrappers, direct meta commands)
- [x] Published to crates.io (`cargo install monorepo-meta`)

### v0.3.1 ✅
- [x] `meta dev:stop` command to stop all tmux development sessions
- [x] Shared Rust config files at workspace root (rustfmt.toml, .clippy.toml, cargo-sort.toml)
- [x] Improved tmux navigation guide with keyboard shortcuts

### v0.4.1 ✅
- [x] `meta status` command for process monitoring
- [x] Lifecycle logging to `.meta/logs/dev.log` (START/EXIT/RESTART events)
- [x] Binary staleness detection (warns if binary rebuilt but process not restarted)

### v0.5.0 ✅
- [x] Per-project log capture (stdout/stderr to `.meta/logs/<project>.log`)
- [x] `meta logs` command with `--follow` and `--lines` options
- [x] Log rotation (10MB threshold, single backup)
- [x] Project validation in logs command with helpful suggestions

### v0.6.0 (Current) ✅
- [x] **Claude Code Skill Redesign** - AI-optimized skill with decision trees, output parsing schemas, error taxonomy, and workflow templates
- [x] Multi-instance support (directory-based session names like `meta-<workspace>`)
- [x] Fixed process detection using tmux pane queries instead of unreliable `ps aux` parsing
- [x] `meta sessions` command to list all active meta sessions across workspaces

### v0.7.0 (Next)
- [ ] Session management (save/restore pane layouts)
- [ ] Multiple environment support (dev, staging, prod)
- [ ] Project dependency awareness (start in order)

### Future
- [ ] Remote execution (SSH to dev servers)
- [ ] Task execution metrics
- [ ] Custom tool adapters via plugins
- [ ] Integration with CI/CD pipelines

## Development

```bash
# Run meta in development
cargo run -- dev

# Run tests
cargo test

# Build release binary
cargo build --release

# Install locally
cargo install --path .
```

### Quality Policy

**Meta adheres to a strict zero-warning, zero-failure policy:**

- ✅ All builds must complete without warnings
- ✅ All tests must pass
- ✅ Code formatted with `rustfmt`
- ✅ Linted with `clippy -- -D warnings`
- ✅ Security audit passes
- ✅ All quality gates enforced in CI/CD

**Before committing:**

```bash
# Run quality checks using meta itself (dogfooding!)
cd tooling/meta
meta run fmt          # Check formatting
meta run clippy       # Lint with warnings as errors
meta test             # Run all tests
meta run audit        # Security audit

# Verify zero-warning build
cargo build --release
```

### Available Meta Commands (Dogfooding)

Meta uses itself for quality gates! All commands run from `tooling/meta` directory:

```bash
meta run fmt          # Check formatting
meta run fmt-fix      # Auto-format code
meta run clippy       # Run linter (zero warnings)
meta run clippy-fix   # Auto-fix lint issues
meta run audit        # Security audit
meta run check        # Fast compile check
meta build --prod     # Build release binary
meta test             # Run all tests
```

## 🤝 Contributing

All contributions must pass our quality gates:

### Quality Gates Enforced

1. **Code Formatting** (`cargo fmt --check`)
   - All code must be formatted with rustfmt
   - Configuration in `rustfmt.toml`

2. **Linting** (`cargo clippy -- -D warnings`)
   - Zero clippy warnings allowed
   - Configuration in `.clippy.toml`

3. **Testing** (`cargo test`)
   - All tests must pass
   - Integration and unit tests

4. **Security Audit** (`cargo audit`)
   - No known vulnerabilities

5. **Cross-Platform** (CI)
   - Tests on Ubuntu, macOS, Windows
   - Rust stable and beta

### CI/CD Pipeline

Our GitHub Actions workflow (`.github/workflows/meta-ci.yml`) runs on every PR:

- ✅ **Tests** - On Ubuntu, macOS, Windows with stable and beta Rust
- ✅ **Formatting Check** - Ensures code is formatted
- ✅ **Clippy Linting** - Zero warnings policy
- ✅ **Security Audit** - Checks for vulnerabilities
- ✅ **Build Artifacts** - Produces binaries for all platforms
- ✅ **Code Coverage** - Tracks test coverage with Codecov

### PR Checklist

Before submitting a PR:

- [ ] Run `make pre-commit` successfully
- [ ] Add tests for new features
- [ ] Update documentation if needed
- [ ] Add entry to CHANGELOG.md (if exists)
- [ ] Ensure no new warnings or errors
- [ ] Verify builds on your platform

## License

MIT

## Credits

Inspired by:
- [bacon](https://github.com/Canop/bacon) - Amazing Rust dev tool
- [Turborepo](https://turbo.build) - Fast monorepo builds
- [moon](https://moonrepo.dev) - Polyglot monorepo tool

---

**Built with ❤️ in Rust**
