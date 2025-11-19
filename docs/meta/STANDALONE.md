# Meta - Standalone Installation Guide

**Version:** 0.2.0
**License:** MIT

---

## Overview

**Meta** is a unified task orchestrator for monorepos that combines multiple tools (Turborepo, Cargo, Bacon) under one interface with real-time log streaming.

This guide shows how to install and use meta **standalone** - independent of the rust-v1 monorepo example.

---

## Features

- 🎯 **Unified CLI** - One command for all development tasks
- 🖥️ **Interactive TUI** - Real-time log streaming with color coding
- 🔍 **Log filtering** - Focus on specific projects
- 📜 **Scrollback** - Navigate through log history
- ⚡ **Fast** - Async Rust = minimal overhead (~5 MB memory)
- 🚀 **Zero config** - Works with sensible defaults

---

## Prerequisites

- **Rust** 1.70+ - Install from [rustup.rs](https://rustup.rs/)
- **Cargo** - Comes with Rust

Optional (for full functionality):
- **Bacon** - Rust hot-reload: `cargo install bacon`
- **Turborepo** - TypeScript builds: `npm install -g turbo`

---

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone meta source (entire monorepo or just tooling/meta directory)
git clone https://github.com/wolven-tech/rust-v1.git
cd rust-v1/tooling/meta

# Quick install with script
./install.sh

# Or manual install
cargo install --path .

# Verify installation
meta --version
```

**Note:** You can copy just the `tooling/meta` directory to any location and run the install script from there.

### Option 2: From crates.io (Future)

```bash
# Once published to crates.io
cargo install meta-orchestrator

# Verify
meta --version
```

### Option 3: From Binary Release (Future)

```bash
# Download from GitHub releases
curl -L https://github.com/wolven-tech/rust-v1/releases/download/v0.2.0/meta-{platform} -o meta

# Make executable
chmod +x meta

# Move to PATH
mv meta ~/.local/bin/  # or /usr/local/bin
```

---

## Updating Meta

### Update from Source

```bash
# Navigate to meta source directory
cd path/to/meta

# Pull latest changes
git pull origin main

# Rebuild and reinstall
cargo install --path .

# Verify new version
meta --version
```

### Update from crates.io (Future)

```bash
# Once published to crates.io
cargo install meta-orchestrator --force

# Verify new version
meta --version
```

### Check for Updates

```bash
# Check current version
meta --version

# Check latest release on GitHub
curl -s https://api.github.com/repos/wolven-tech/rust-v1/releases/latest | grep "tag_name"
```

---

## Quick Start

### 1. Initialize Configuration

```bash
# In your monorepo root
meta init

# This creates meta.toml with default configuration
```

### 2. Configure Your Projects

Edit `meta.toml`:

```toml
[workspace]
name = "My Monorepo"
root = "."

[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript", "javascript"]
for_tasks = ["dev", "build"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]
for_tasks = ["build", "test"]

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
test = { tool = "cargo", command = "test" }

[projects.web]
type = "typescript"
path = "apps/web"
[projects.web.tasks]
dev = { tool = "turborepo", command = "dev --filter=@yourorg/web" }
build = { tool = "turborepo", command = "build --filter=@yourorg/web" }
```

### 3. Start Development

```bash
# Launch TUI with log streaming
meta tui

# Or start without TUI
meta dev

# Or specific projects
meta dev -p api -p web
```

---

## Usage

### Commands

```bash
# Initialize configuration
meta init

# Start all development servers
meta dev

# Start specific projects
meta dev -p api -p web

# Build all projects
meta build

# Production build
meta build --prod

# Run tests
meta test

# Interactive TUI
meta tui
```

### TUI Keyboard Shortcuts

| Key | Action | Description |
|-----|--------|-------------|
| `q` | Quit | Exit the TUI |
| `↑/k`, `↓/j` | Navigate | Select project |
| `PgUp/PgDn` | Scroll | Scroll through logs |
| `Home` | Jump to top | Go to first log |
| `End` | Jump to bottom | Go to latest log + auto-scroll |
| `Enter` | Toggle filter | Show only selected project's logs |
| `a` | Show all | Remove filter |
| `c` | Clear | Clear log buffer |

### Log Features

- **Auto-scroll mode** (default) - Automatically follows new logs
- **Manual scroll mode** - Use PgUp/PgDn to browse history
- **Color coding**:
  - White - Info messages
  - Red - Errors
  - Dark gray - Debug messages
- **Copyable** - Use terminal selection to copy logs
- **1000-line buffer** - Keeps last 1000 log lines

---

## Configuration

### Workspace Settings

```toml
[workspace]
name = "My Monorepo"    # Display name
root = "."              # Workspace root directory
```

### Tool Configuration

Define which tools are available:

```toml
[tools.{tool-name}]
enabled = true
command = "command-name"
for_languages = ["lang1", "lang2"]
for_tasks = ["dev", "build", "test"]
```

### Project Configuration

Define each project:

```toml
[projects.{project-name}]
type = "rust" | "typescript" | "javascript" | "deno"
path = "path/to/project"

[projects.{project-name}.tasks]
dev = { tool = "tool-name", command = "command args" }
build = { tool = "tool-name", command = "command args" }
test = { tool = "tool-name", command = "command args" }
```

---

## Examples

### Rust + TypeScript Monorepo

```toml
[workspace]
name = "fullstack-app"
root = "."

[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript"]
for_tasks = ["dev", "build", "test"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]
for_tasks = ["dev"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]
for_tasks = ["build", "test"]

[projects.api]
type = "rust"
path = "apps/api"
[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
test = { tool = "cargo", command = "test" }

[projects.web]
type = "typescript"
path = "apps/web"
[projects.web.tasks]
dev = { tool = "turborepo", command = "dev" }
build = { tool = "turborepo", command = "build" }
test = { tool = "turborepo", command = "test" }
```

### Multi-Language Monorepo

```toml
[workspace]
name = "polyglot-app"

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]

[tools.deno]
enabled = true
command = "deno"
for_languages = ["typescript"]

[tools.go]
enabled = true
command = "go"
for_languages = ["go"]

[projects.rust-api]
type = "rust"
path = "services/api"
[projects.rust-api.tasks]
dev = { tool = "cargo", command = "watch -x run" }
build = { tool = "cargo", command = "build --release" }

[projects.deno-worker]
type = "typescript"
path = "workers/processor"
[projects.deno-worker.tasks]
dev = { tool = "deno", command = "task dev" }

[projects.go-service]
type = "go"
path = "services/auth"
[projects.go-service.tasks]
dev = { tool = "go", command = "run ." }
build = { tool = "go", command = "build" }
```

---

## Integrating with Existing Projects

### Quick Setup for Any Monorepo

```bash
# 1. Copy meta to your monorepo (optional, can also use globally installed meta)
cp -r path/to/rust-v1/tooling/meta your-monorepo/tools/meta
cd your-monorepo/tools/meta
./install.sh

# 2. In your monorepo root
cd your-monorepo
meta init

# 3. Edit meta.toml to configure your projects
# 4. Start using meta
meta dev
```

### Step 1: Add meta.toml

Create `meta.toml` in your monorepo root with your existing structure.

### Step 2: Map Existing Scripts

If you have npm scripts:

```json
{
  "scripts": {
    "dev:api": "cd apps/api && cargo watch -x run",
    "dev:web": "turbo dev --filter=web"
  }
}
```

Map them to meta:

```toml
[projects.api.tasks]
dev = { tool = "cargo", command = "watch -x run" }

[projects.web.tasks]
dev = { tool = "turborepo", command = "dev --filter=web" }
```

### Step 3: Replace Scripts (Optional)

Update package.json to use meta:

```json
{
  "scripts": {
    "dev": "meta dev",
    "build": "meta build --prod",
    "test": "meta test"
  }
}
```

---

## Troubleshooting

### meta: command not found

```bash
# Ensure cargo bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Add to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

### Tool not found errors

```bash
# Install missing tools
cargo install bacon    # For Rust hot-reload
npm install -g turbo   # For TypeScript
```

### Configuration errors

```bash
# Check meta.toml syntax
meta init  # Re-run to get defaults

# Validate with specific project
meta dev -p your-project
```

---

## Quality Policy

**Meta adheres to zero-warning, zero-failure policy:**

- ✅ All builds must complete without warnings
- ✅ All tests must pass
- ✅ Code formatted with `rustfmt`
- ✅ Linted with `clippy -- -D warnings`

---

## Performance

| Metric | Value |
|--------|-------|
| Binary size | 2.7 MB (release) |
| Memory usage | ~5 MB (idle) |
| CPU usage | <5% (active logging) |
| Startup time | <50ms |
| Log throughput | 3000+ lines/sec |

---

## Development

### Building from Source

```bash
git clone https://github.com/wolven-tech/rust-v1.git
cd rust-v1/tooling/meta

# Development build
cargo build

# Run tests
cargo test

# Release build
cargo build --release

# Install locally
cargo install --path .
```

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name
```

### Formatting & Linting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint
cargo clippy

# Lint with warnings as errors
cargo clippy -- -D warnings
```

---

## Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch
3. Make changes
4. Ensure zero warnings: `cargo clippy -- -D warnings`
5. Format code: `cargo fmt`
6. Run tests: `cargo test`
7. Submit a pull request

---

## Roadmap

### v0.3.0 (Planned)
- [ ] Watch mode - Auto-restart on file changes
- [ ] Log search - Regex pattern matching
- [ ] Log export - Save logs to file

### v0.4.0 (Planned)
- [ ] Task history - Track execution times
- [ ] Performance metrics - CPU/memory per project
- [ ] Plugin system - Extensible architecture

### v1.0.0 (Future)
- [ ] Remote execution - SSH support
- [ ] Web dashboard - Browser-based UI
- [ ] CI/CD templates - Pre-built workflows

---

## FAQ

**Q: Can I use meta without the example monorepo?**
A: Yes! Meta is standalone. Just install and configure meta.toml.

**Q: Does meta replace Turborepo/Cargo?**
A: No, meta orchestrates them. It provides a unified interface.

**Q: Can I use meta with my existing CI/CD?**
A: Yes! Use `meta build --prod` and `meta test` in your pipeline.

**Q: How do I copy logs from the TUI?**
A: Use your terminal's selection mode (usually mouse or keyboard shortcuts).

**Q: Does meta support Windows?**
A: Yes, Rust and Crossterm are cross-platform.

---

## Support

- **Issues:** [GitHub Issues](https://github.com/wolven-tech/rust-v1/issues)
- **Documentation:** [Full Docs](https://github.com/wolven-tech/rust-v1/tree/main/docs)
- **Repository:** [rust-v1 monorepo](https://github.com/wolven-tech/rust-v1)

---

## License

MIT License - See [LICENSE](LICENSE) for details.

---

## Acknowledgments

- Inspired by [Bacon](https://github.com/Canop/bacon)
- Built with [Ratatui](https://ratatui.rs)
- Powered by [Tokio](https://tokio.rs)

---

**Happy coding with Meta! 🚀**

Use `meta tui` for the best development experience.
