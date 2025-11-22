# Using Meta in Your Monorepo

This guide shows you how to use **meta** in your own monorepo project.

## Demo

See Meta in action:

[![asciicast](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq.svg)](https://asciinema.org/a/N9NaULQ470ih2SV6aymeENmLq)

## What is Meta?

Meta is a **monorepo task orchestrator** that launches multiple bacon and turborepo instances simultaneously in tmux, giving you:

- 🥓 **Multiple Bacon TUIs** - Each Rust project runs bacon with full interactive TUI
- ⚡ **Turborepo Integration** - Proper `--filter` support for Next.js/TypeScript projects
- 🖥️ **Tmux Orchestration** - All processes in separate panes, easy to navigate
- ✅ **Zero Configuration** - Auto-detects projects and generates `meta.toml`

## Prerequisites

### Required
- **Rust & Cargo** - For building meta
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **tmux** - For running multiple processes
  ```bash
  # macOS
  brew install tmux

  # Ubuntu/Debian
  apt install tmux

  # Fedora/RHEL
  yum install tmux
  ```

### Recommended (for best experience)
- **Bacon** - Rust hot-reload development
  ```bash
  cargo install bacon
  ```

- **Turborepo** - For JavaScript/TypeScript monorepos
  ```bash
  npm install -g turbo
  # or
  bun add -g turbo
  ```

## Installation

### Method 1: Install from this Repository (Recommended)

```bash
# Clone or download just the meta directory
git clone https://github.com/YOUR_ORG/YOUR_REPO.git
cd YOUR_REPO/tooling/meta

# Run the install script
chmod +x install.sh
./install.sh

# Verify installation
meta --version
```

### Method 2: Manual Installation

```bash
# From the meta directory
cargo install --path .

# Or install from a specific commit
cargo install --git https://github.com/YOUR_ORG/YOUR_REPO.git --root ~/.cargo meta
```

### Method 3: Copy to Your Monorepo

The `tooling/meta` directory is self-contained. You can copy it directly:

```bash
# Copy meta to your monorepo
cp -r path/to/this/tooling/meta your-monorepo/tooling/meta

# Build and use locally (no global install)
cd your-monorepo
cargo run --manifest-path tooling/meta/Cargo.toml -- init
```

## Quick Start

### 1. Initialize Configuration

Navigate to your monorepo root and initialize:

```bash
cd your-monorepo/
meta init
```

This auto-detects projects and creates `meta.toml`:
- Scans `apps/` and `packages/` directories
- Detects Rust projects (Cargo.toml)
- Detects Next.js projects (package.json with "next" dependency)
- Generates tasks for each project

### 2. Validate Setup

```bash
meta doctor
```

This checks:
- ✅ All tools are installed (bacon, cargo, turbo, tmux)
- ✅ meta.toml is valid
- ✅ All project paths exist
- ✅ Tasks are properly configured

### 3. Start Development

```bash
meta dev
```

This launches a tmux session with:
- One pane per project
- Each bacon instance with full interactive TUI
- Each turbo command running from workspace root
- All processes visible and manageable

**Tmux Controls:**
- `Ctrl+B` then `D` - Detach (processes keep running)
- `Ctrl+B` then arrow keys - Navigate between panes
- `Ctrl+C` in a pane - Stop that process
- `tmux attach -t meta-dev` - Reattach to session

## Configuration

### meta.toml Structure

```toml
version = "1"

[workspace]
name = "My Monorepo"
root = "."

# Tool declarations
[tools.turborepo]
enabled = true
command = "turbo"
for_languages = ["typescript", "javascript"]
for_tasks = ["dev", "build", "lint", "typecheck"]

[tools.bacon]
enabled = true
command = "bacon"
for_languages = ["rust"]
for_tasks = ["dev"]

[tools.cargo]
enabled = true
command = "cargo"
for_languages = ["rust"]
for_tasks = ["build", "test", "clippy"]

# Project definitions
[projects.api]
type = "rust"
path = "apps/api"

[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
build = { tool = "cargo", command = "build --release" }
test = { tool = "cargo", command = "test" }

[projects.web]
type = "next"
path = "apps/web"

[projects.web.tasks]
dev = { tool = "turborepo", command = "run dev --filter=@your-org/web" }
build = { tool = "turborepo", command = "run build --filter=@your-org/web" }
```

### Important Configuration Notes

**Turborepo Commands:**
- Must include `run` keyword: `run dev --filter=...`
- Use exact package name from package.json: `--filter=@your-org/package-name`
- Always run from workspace root (meta handles this automatically)

**Bacon Commands:**
- Use the job name from bacon.toml: `run-long`, `check`, `clippy`
- Meta changes to project directory before running bacon

**Tool Routing:**
- Turborepo: Runs from workspace root
- Bacon/Cargo: Changes to project directory first

## Usage Examples

### Run All Dev Servers

```bash
meta dev
```

### Run Specific Projects

```bash
meta dev --projects api web
```

### Run Tasks Across Projects

```bash
# Run tests for all projects
meta run test

# Run clippy for specific projects
meta run clippy --projects api backend
```

### Build All Projects

```bash
meta build
```

## Troubleshooting

### Tmux Session Won't Start

**Issue:** `open terminal failed: not a terminal`

**Solution:** Make sure you're running in a real terminal, not through an IDE or automation tool.

### Turbo Commands Exit Immediately

**Issue:** Turbo panes close right after starting

**Possible causes:**
1. Missing `run` keyword in command
2. Wrong package filter name
3. Missing dependencies in the app

**Check with:**
```bash
meta doctor  # Validates turbo command syntax
```

### Bacon Not Found

**Issue:** `bacon command failed`

**Solution:**
```bash
cargo install bacon
meta doctor  # Verify installation
```

### Projects Not Detected

**Issue:** `meta init` doesn't find your projects

**Solution:** Manually add them to `meta.toml`:
```toml
[projects.your-app]
type = "rust"  # or "next"
path = "path/to/your-app"

[projects.your-app.tasks]
dev = { tool = "bacon", command = "check" }
```

## Integration with Existing Tools

### With Bacon

Meta works alongside bacon by launching it properly:
- Each bacon instance gets its own tmux pane
- Full keyboard interaction available
- Can switch jobs (t for test, c for clippy, etc.)

### With Turborepo

Meta respects turborepo's architecture:
- Runs from workspace root
- Uses proper `--filter` syntax
- Leverages turbo's caching and parallelization

### With Package.json Scripts

Add meta commands to your root package.json:

```json
{
  "scripts": {
    "dev": "meta dev",
    "build": "meta build",
    "test": "meta test",
    "doctor": "meta doctor"
  }
}
```

## Advanced Configuration

### Custom Tasks

Add any task to any project:

```toml
[projects.api.tasks]
dev = { tool = "bacon", command = "run-long" }
format = { tool = "cargo", command = "fmt --all" }
lint = { tool = "cargo", command = "clippy --all-targets -- -D warnings" }
audit = { tool = "cargo", command = "audit" }
```

Then run with:
```bash
meta run format
meta run lint --projects api
```

### Multiple Workspaces

For monorepos with multiple workspace roots:

```toml
[projects.backend-api]
type = "rust"
path = "backend/api"

[projects.backend-jobs]
type = "rust"
path = "backend/jobs"

[projects.frontend-web]
type = "next"
path = "frontend/web"
```

### Excluding Projects from Dev

Remove the `dev` task from projects you don't want to run:

```toml
[projects.docs.tasks]
# No dev task - won't appear in `meta dev`
build = { tool = "cargo", command = "build" }
```

## Best Practices

1. **Version Control** - Commit `meta.toml` to your repository
2. **Team Consistency** - Everyone uses the same dev setup
3. **Validate Often** - Run `meta doctor` after config changes
4. **Use Filters** - Start only what you need: `meta dev --projects api`
5. **Leverage Tmux** - Learn basic tmux navigation for better workflow

## Getting Help

```bash
meta --help        # General help
meta dev --help    # Command-specific help
meta doctor        # Validate your setup
```

## Updating Meta

```bash
# If installed globally
cargo install --path /path/to/meta --force

# If copied to your monorepo
cd your-monorepo/tooling/meta
git pull  # or copy updated files
cargo build --release
```

## License

MIT - See LICENSE file for details
