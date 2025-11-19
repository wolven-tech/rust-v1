# rust-v1 🚀

> Modern full-stack monorepo template with Rust + Next.js, powered by the **meta orchestrator** with real-time log streaming

**🚀 Quick Start:** `bunx degit wolven-tech/rust-v1 my-project`

A production-ready monorepo template featuring:
- 🦀 **Rust API** (Axum with Clean Architecture)
- ⚛️ **Next.js Apps** (Marketing + Application)
- 🎯 **Meta Orchestrator** (Unified CLI with TUI dashboard)
- 🖥️ **Real-time Log Streaming** (Color-coded, filterable)
- ⚡ **Parallel Execution** (Async process management)

## ✨ What Makes Meta Awesome?

**Meta** is your unified development command center. One tool to rule them all:

```bash
# Interactive TUI with live log streaming
meta tui

# Start all development servers
meta dev

# Build everything for production
meta build --prod

# Run all tests
meta test
```

### Key Features

- 🎨 **Color-coded logs** - Instantly spot errors (red) vs info (white)
- 🔍 **Filter by project** - Focus on what matters (press `Enter`)
- 🚀 **Zero config** - Works out of the box with sensible defaults
- ⚡ **Fast** - Async Rust = minimal overhead (~5 MB memory)
- 📊 **Visual feedback** - Project status, timestamps, live updates

---

## 🚀 Quick Start

### Create Your Own Project

Use `degit` to scaffold a new project from this monorepo:

```bash
# Create a new project from this template
bunx degit wolven-tech/rust-v1 my-project
cd my-project

# Install dependencies
bun install

# Install meta orchestrator
cd tooling/meta && ./install.sh && cd ../..

# Start development with TUI
meta tui
```

That's it! 🎉

### Prerequisites

- [Bun](https://bun.sh/) 1.1.26+
- [Rust](https://rustup.rs/) 1.70+
- [Bacon](https://github.com/Canop/bacon) - `cargo install bacon`

### Clone for Development

```bash
# Clone the repository
git clone https://github.com/wolven-tech/rust-v1.git
cd rust-v1

# Install dependencies
bun install

# Install meta orchestrator
cd tooling/meta && ./install.sh && cd ../..

# Start development
meta tui
```

**Want to use meta in your own project?** Check out the **[Standalone Installation Guide](docs/meta/STANDALONE.md)** to install meta independently.

---

## 💻 Development

### The Meta Way (Recommended)

```bash
# Interactive dashboard with log streaming
meta tui

# Start all servers
meta dev

# Start specific projects
meta dev -p api -p web

# Build for production
meta build --prod

# Run tests
meta test
```

### TUI Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `↑/k`, `↓/j` | Navigate projects |
| `Enter` | Toggle log filter for selected project |
| `a` | Show all logs (remove filter) |
| `c` | Clear log buffer |
| `q` | Quit |

### Traditional npm Scripts (Still Available)

```bash
# Code quality
bun lint                # Run linters
bun typecheck           # TypeScript checks
bun format              # Format code

# Production
bun start:web           # Start web in production
bun start:app           # Start app in production

# Cleanup
bun clean               # Clean node_modules
bun clean:workspaces    # Clean turbo cache
```

**Note:** Development workflows (`dev`, `build`, `test`) have been replaced by `meta` for a better unified experience.

---

## 📁 Project Structure

```
rust-v1/
├── apps/
│   ├── api/              # 🦀 Rust API (Axum + Clean Architecture)
│   ├── web/              # ⚛️ Next.js marketing site
│   └── app/              # ⚛️ Next.js application
│
├── packages/             # 8 shared packages
│   ├── ui/               # React components (Tailwind)
│   ├── supabase/         # Database client
│   ├── analytics/        # PostHog integration
│   └── ...
│
├── tooling/
│   ├── meta/             # 🎯 Meta orchestrator (v0.2.0)
│   ├── typescript/       # Shared TS config
│   └── e2e/              # Playwright tests
│
├── docs/                 # 📚 Documentation
│   ├── meta/             # Meta-specific docs
│   ├── guides/           # Tutorials & examples
│   └── releases/         # Version notes
│
├── meta.toml             # Meta configuration
├── turbo.json            # Turborepo config
└── README.md             # This file
```

---

## 📚 Documentation

### Getting Started
- **[Quick Start](#-quick-start)** - You're reading it!
- **[Examples](docs/guides/EXAMPLES.md)** - 30+ practical usage examples
- **[Getting Started Guide](docs/guides/GETTING_STARTED.md)** - Detailed onboarding

### Meta Orchestrator
- **[Meta README](tooling/meta/README.md)** - Features, CLI, configuration
- **[Log Streaming](docs/meta/LOG_STREAMING.md)** - Implementation details
- **[Release Notes v0.2.0](docs/releases/RELEASE_v0.2.0.md)** - Latest features

### Architecture
- **[Documentation Index](docs/INDEX.md)** - Complete doc navigation
- **[Architecture](docs/current/ARCHITECTURE.md)** - System design
- **[Monorepo Structure](docs/current/MONOREPO_STRUCTURE.md)** - Organization

### Reference
- **[CHANGELOG](CHANGELOG.md)** - Version history
- **[API Documentation](apps/api/README.md)** - Rust API details

---

## 🛠️ Tech Stack

### Meta Orchestrator
- **Rust** - Performance & safety
- **Clap** - CLI argument parsing
- **Ratatui** - Terminal UI framework
- **Tokio** - Async runtime
- **Crossterm** - Terminal control

### Frontend
- **Next.js 15** - React framework
- **React 19** - UI library
- **TypeScript** - Type safety
- **Tailwind CSS** - Styling
- **Turborepo** - Build system

### Backend
- **Rust** - Systems programming
- **Axum** - Web framework
- **Clean Architecture** - 4-layer design
- **OpenAPI** - API documentation

### Development Tools
- **Bun** - Package manager & runtime
- **Bacon** - Rust hot-reload
- **Biome** - Linting & formatting
- **Playwright** - E2E testing

---

## 🎯 Common Tasks

### Development Workflow

```bash
# Start everything with TUI
meta tui

# Or start without TUI
meta dev

# Watch specific projects
meta dev -p api        # Rust API only
meta dev -p web app    # Both Next.js apps
```

### Building

```bash
# Development build
meta build

# Production build
meta build --prod

# Specific projects
meta build -p api --prod
```

### Testing

```bash
# Run all tests
meta test

# Future: watch mode
meta test -w           # Coming in v0.3.0
```

### Code Quality

```bash
# Format code
bun format

# Run linters
bun lint

# Type check
bun typecheck

# Fix repository issues
bun lint:repo:fix
```

---

## 🚢 Deployment

### Production Build

```bash
# Build all projects
meta build --prod

# Verify builds
ls -lh apps/api/target/release/api    # ~6 MB
ls -lh apps/web/.next                  # Next.js build
```

### Running in Production

```bash
# Rust API
cd apps/api
./target/release/api

# Next.js apps
bun start:web    # Marketing site
bun start:app    # Application
```

---

## 📊 Performance

### Meta Orchestrator

| Metric | Value |
|--------|-------|
| Binary size | 2.7 MB (release) |
| Memory usage | ~5 MB (idle) |
| CPU usage | <5% (active logging) |
| Startup time | <50ms |
| Throughput | 3000+ lines/sec |

### Rust API

| Metric | Before (TypeScript) | After (Rust) | Improvement |
|--------|---------------------|--------------|-------------|
| Memory | 30-50 MB | 2-5 MB | **6-10x less** |
| Startup | ~500ms | <100ms | **5x faster** |
| Binary | ~50 MB | 6.3 MB | **8x smaller** |

---

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test with meta: `meta build --prod && meta test`
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Use `meta` for development workflows
- Format code before committing: `bun format`
- Ensure tests pass: `meta test`
- Update documentation as needed

### Quality Standards

**This project maintains a zero-warning, zero-failure policy:**

- ✅ **Meta (Rust):** Zero warnings, all tests pass
- ✅ **TypeScript:** No type errors, linting passes
- ✅ **Formatting:** All code formatted (Biome/rustfmt)

**For Rust contributions:**

```bash
# Meta quality checks
cd tooling/meta
cargo fmt -- --check          # Verify formatting
cargo clippy -- -D warnings   # Lint with warnings as errors
cargo test                    # Run all tests
cargo build --release         # Zero-warning build
```

**For TypeScript contributions:**

```bash
bun format       # Format code
bun lint         # Run linters
bun typecheck    # Type checking
```

---

## 🎉 What's New

### v0.2.0 (2025-01-19)

- ✨ **Real-time log streaming** in TUI
- 🎨 **Color-coded output** (info=white, error=red)
- 🔍 **Log filtering** by project
- ⚡ **High performance** (~3000 lines/sec)
- 📝 **Comprehensive documentation**

See [CHANGELOG](CHANGELOG.md) for complete history.

---

## 🔗 Links

- **Documentation:** [docs/INDEX.md](docs/INDEX.md)
- **Examples:** [docs/guides/EXAMPLES.md](docs/guides/EXAMPLES.md)
- **Meta CLI:** [tooling/meta/README.md](tooling/meta/README.md)
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)
- **License:** [LICENSE.md](LICENSE.md)

---

## 📧 Support

- **Issues:** Report bugs via GitHub issues
- **Questions:** Check [docs/INDEX.md](docs/INDEX.md) first
- **Contributing:** See [Contributing](#-contributing) above

---

## 📜 License

MIT License - See [LICENSE.md](LICENSE.md)

---

**Built with ❤️ using Meta**

*Built with ❤️ by the Rust and TypeScript communities*

---

**Happy coding! 🚀**

Use `meta tui` for the best development experience.
