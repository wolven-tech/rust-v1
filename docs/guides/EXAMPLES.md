# Meta Orchestrator Examples

Practical examples of using the meta orchestrator in your development workflow.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Usage](#basic-usage)
3. [Development Workflows](#development-workflows)
4. [Building Projects](#building-projects)
5. [Testing](#testing)
6. [Advanced Configuration](#advanced-configuration)
7. [Real-World Scenarios](#real-world-scenarios)

---

## Getting Started

### Create a New Project

```bash
# Scaffold a new project from this template
bunx degit wolven-tech/rust-v1 my-project
cd my-project

# Install dependencies
bun install

# Install meta orchestrator
cd tooling/meta && ./install.sh && cd ../..

# Start development
meta tui
```

### Clone for Development

```bash
# Clone the repository
git clone https://github.com/wolven-tech/rust-v1.git
cd rust-v1

# Install dependencies
bun install

# Install meta
cd tooling/meta && ./install.sh && cd ../..

# Start development
meta tui
```

---

## Basic Usage

### Initialize Meta

```bash
# Create meta.toml configuration in your own project
meta init

# Or use npm script
bun meta:init
```

### Start Development

```bash
# Start all development servers
meta dev

# Start specific projects
meta dev -p api
meta dev -p web -p app
meta dev -p api -p web -p app
```

### Build Projects

```bash
# Development build
meta build

# Production build
meta build --prod

# Build specific projects
meta build -p api
meta build --prod -p api -p web
```

### Run Tests

```bash
# Run all tests
meta test

# Watch mode (coming soon)
meta test -w
```

### Launch TUI

```bash
# Interactive dashboard with real-time log streaming
meta tui

# Features:
# - Real-time logs from all running dev servers
# - Color-coded output (info=white, error=red)
# - Filter logs by project (select project and press Enter)
# - Clear logs (press c)
# - Show all logs (press a)
# - Navigate with j/k or ↑/↓
# - Quit with q
```

---

## Development Workflows

### Full-Stack Development

Start Rust API + Next.js apps simultaneously:

```bash
# Option 1: Meta orchestrator
meta dev

# Option 2: npm script
bun meta:dev

# What this does:
# - Starts apps/api with bacon (hot-reload)
# - Starts apps/web with turbo dev
# - Starts apps/app with turbo dev
# - All running in parallel!
```

### API-Only Development

```bash
# Just the Rust API
meta dev -p api

# Or use bacon directly
cd apps/api && bacon run-long
```

### Frontend-Only Development

```bash
# Just Next.js apps
meta dev -p web -p app

# Or use turbo directly
bun dev:web
bun dev:app
```

### Selective Development

```bash
# API + one frontend
meta dev -p api -p web

# All frontends, no API
meta dev -p web -p app
```

---

## Building Projects

### Development Build

```bash
# Build everything (debug mode)
meta build

# What this does:
# - cargo build for api
# - turbo build for web/app
```

### Production Build

```bash
# Optimized builds
meta build --prod

# What this does:
# - cargo build --release for api
# - turbo build (production) for web/app
# - Applies all optimizations
```

### Incremental Builds

```bash
# Build only what changed
meta build -p api

# Turborepo handles caching automatically
# Cargo does incremental compilation
```

### CI/CD Build

```bash
# Full production build (all projects)
meta build --prod

# Verify builds
ls -lh apps/api/target/release/api
ls -lh apps/web/.next
```

---

## Testing

### Run All Tests

```bash
# Execute test suite across all projects
meta test

# What this does:
# - cargo test for api
# - turbo test for web/app
# - Runs in parallel
```

### Project-Specific Tests

```bash
# Test only API
cd apps/api && cargo test

# Test only web
cd apps/web && bun test

# Test with meta (future)
meta test -p api
```

### Watch Mode

```bash
# Auto-run tests on change (coming soon)
meta test -w

# For now, use tool-specific watch:
cd apps/api && cargo watch -x test
cd apps/web && bun test --watch
```

---

## Advanced Configuration

### Custom Workflows

Add to `meta.toml`:

```toml
# Development workflow
[workflows.dev-all]
description = "Start all development servers"
parallel = true
projects = ["api", "web", "app"]

# Backend only
[workflows.dev-backend]
description = "Start API and dependencies"
parallel = true
projects = ["api"]

# Frontend only
[workflows.dev-frontend]
description = "Start all frontends"
parallel = true
projects = ["web", "app"]

# Production build
[workflows.build-prod]
description = "Production build pipeline"
parallel = false  # Sequential
projects = ["ui", "web", "app", "api"]

# Run then:
meta workflow dev-backend
meta workflow build-prod
```

### Custom Tools

```toml
[tools.deno]
enabled = true
command = "deno"
for_languages = ["typescript"]
for_tasks = ["dev", "test"]

[projects.deno-service]
type = "deno"
path = "apps/deno-service"
[projects.deno-service.tasks]
dev = { tool = "deno", command = "task dev" }
```

### Environment-Specific Config

```toml
[tools.turborepo.dev]
command = "turbo"
env = { NODE_ENV = "development" }

[tools.turborepo.prod]
command = "turbo"
env = { NODE_ENV = "production" }
```

---

## Real-World Scenarios

### Scenario 1: New Developer Onboarding

```bash
# Clone repo
git clone <repo>
cd rust-v1

# Install dependencies
bun install

# Install meta
cd tooling/meta && ./install.sh

# Start everything
meta dev

# That's it! Everything running.
```

### Scenario 2: Feature Development

```bash
# Create feature branch
git checkout -b feature/user-auth

# Start relevant services
meta dev -p api -p web

# Make changes...
# Hot-reload handles updates automatically

# Run tests
meta test

# Build for production
meta build --prod

# Commit
git add .
git commit -m "feat: add user authentication"
```

### Scenario 3: Debugging API Issues

```bash
# Start only API with full logging
cd apps/api
RUST_LOG=debug bacon run-long

# Or with meta (future):
meta dev -p api --log-level debug
```

### Scenario 4: CI/CD Pipeline

```bash
# In GitHub Actions
- name: Build all projects
  run: |
    cd tooling/meta
    cargo run -- build --prod

- name: Run tests
  run: |
    cd tooling/meta
    cargo run -- test
```

### Scenario 5: Monorepo Migration

```bash
# Before: Multiple terminal tabs
Terminal 1: cd apps/api && cargo watch -x run
Terminal 2: cd apps/web && npm run dev
Terminal 3: cd apps/app && npm run dev

# After: One command
Terminal 1: meta dev
```

### Scenario 6: Selective Development

```bash
# Working on API only
meta dev -p api

# Working on frontend + API integration
meta dev -p api -p web

# Working on all frontends
meta dev -p web -p app
```

### Scenario 7: Performance Testing

```bash
# Build optimized binaries
meta build --prod

# Run benchmarks
cd apps/api && cargo bench

# Load test
wrk -t12 -c400 -d30s http://localhost:4400/health
```

---

## Tips & Tricks

### Fast Iterations

```bash
# Bacon for Rust (sub-second rebuilds)
meta dev -p api
# Saves on every file change!

# Turborepo for TS (cached builds)
meta build
# Only rebuilds what changed
```

### Resource Management

```bash
# Limit projects to save resources
meta dev -p api  # Just API

# Use TUI to see resource usage (future)
meta tui
```

### Debugging

```bash
# Verbose logging
RUST_LOG=meta=debug meta dev

# Check configuration
cat meta.toml

# Test specific project
meta dev -p api
```

### CI Optimization

```yaml
# Cache meta builds
- uses: actions/cache@v4
  with:
    path: tooling/meta/target
    key: meta-${{ hashFiles('tooling/meta/Cargo.lock') }}

# Run meta
- run: cd tooling/meta && cargo run -- build --prod
```

---

## Keyboard Shortcuts (TUI Mode)

```
q       - Quit
j / ↓   - Select next project
k / ↑   - Select previous project
Enter   - Toggle log filter for selected project
c       - Clear all logs
a       - Show all logs (remove filter)

Coming soon:
r       - Restart selected project
/       - Search logs
?       - Help
```

---

## Common Issues

### Meta not found

```bash
# Install meta
cd tooling/meta && ./install.sh

# Or add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Tool not found

```bash
# Install missing tools
cargo install bacon    # For Rust hot-reload
npm install -g turbo   # For TypeScript builds
```

### Port already in use

```bash
# Kill processes on port
lsof -ti:4400 | xargs kill -9

# Or change port in .env
PORT=3003 meta dev
```

---

## Next Steps

- [Read the full documentation](tooling/meta/README.md)
- [Configure meta.toml](meta.toml)
- [Join the community](#) (coming soon)
- [Report issues](#) (coming soon)

---

**Happy coding with Meta! 🚀**
