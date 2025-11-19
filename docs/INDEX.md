# Meta Monorepo Documentation Index

**Last Updated**: 2025-01-19
**Repository**: rust-v1
**Version**: 0.2.0

---

## 🚀 Quick Start

**New to meta?** Start here:

1. [Getting Started Guide](guides/GETTING_STARTED.md) - Setup and first steps
2. [Meta Orchestrator](meta/README.md) - Unified task runner with TUI
3. [Examples](guides/EXAMPLES.md) - 30+ practical examples

**Create your own project:**
```bash
# Use degit to scaffold from this template
bunx degit wolven-tech/rust-v1 my-project
cd my-project

# Install dependencies
bun install

# Install meta orchestrator
cd tooling/meta && ./install.sh && cd ../..

# Start development with TUI
meta tui
```

---

## 📂 Documentation Structure

### Root Documentation (`/docs`)
```
docs/
├── current/          # Active architecture and structure
├── guides/           # How-to guides and tutorials
├── meta/             # Meta orchestrator documentation
├── releases/         # Version release notes
├── archive/          # Historical docs (deprecated/superseded)
└── images/           # Documentation images and diagrams
```

---

## 📚 Current Documentation

### ⭐ Meta Orchestrator (`meta/`)
The heart of the monorepo - unified task orchestration for Turborepo + Cargo + Bacon.

- ✅ **[Meta README](meta/README.md)** - Features, installation, usage
- ✅ **[Standalone Installation](meta/STANDALONE.md)** - Install meta independently
- ✅ **[Log Streaming](meta/LOG_STREAMING.md)** - Real-time log implementation v0.2.0
- ✅ **[Stack Pruning](meta/PRUNING_ANALYSIS.md)** - npm script migration guide
- ✅ **[Session Summary](meta/SUMMARY_v0.2.0.md)** - v0.2.0 development summary
- ✅ **[Release v0.2.0](releases/RELEASE_v0.2.0.md)** - Latest release notes

**Key Features:**
- 🎯 Unified CLI for all tools
- 🖥️ Interactive TUI with live log streaming
- 🎨 Color-coded log output (info/error)
- 🔍 Filter logs by project
- ⚡ Parallel execution
- 🚀 Zero config (sensible defaults)

**Quick Commands:**
```bash
meta dev              # Start all dev servers
meta tui              # TUI with log streaming
meta build --prod     # Production build
meta test             # Run all tests
```

### Architecture (`current/`)
Up-to-date system architecture and organization.

- ✅ **[ARCHITECTURE](current/ARCHITECTURE.md)** - System design and patterns
- ✅ **[MONOREPO_STRUCTURE](current/MONOREPO_STRUCTURE.md)** - Directory layout

### Guides (`guides/`)
Step-by-step tutorials and practical examples.

- ✅ **[Getting Started](guides/GETTING_STARTED.md)** - Onboarding guide
- ✅ **[Examples](guides/EXAMPLES.md)** - 30+ usage examples for meta
- ✅ **[React Query Guide](guides/REACT_QUERY_GUIDE.md)** - Data fetching patterns

### Releases (`releases/`)
Version history and detailed release notes.

- ✅ **[v0.2.0](releases/RELEASE_v0.2.0.md)** - Real-time log streaming (2025-01-19)
- 📝 **[CHANGELOG](../CHANGELOG.md)** - Complete version history

### Core Documentation
- ✅ **[Root README](../README.md)** - Project overview and quick start
- ✅ **[MCP Servers](MCP.md)** - Claude Code MCP configuration
- ✅ **[Documentation Guide](README.md)** - How to use these docs

---

## 🗄️ Archived Documentation

Historical documentation preserved for reference:

- 📦 **[Final Delivery v0.1.0](archive/FINAL_DELIVERY.md)** - Initial transformation delivery
- 📦 **[Rebranding Summary](archive/REBRANDING_AND_META_SUMMARY.md)** - @v1 → @meta migration

---

## 🏗️ Architecture Overview

### Technology Stack

**Meta Orchestrator:**
- Rust (Clap, Ratatui, Tokio)
- TUI with real-time log streaming
- Async process management

**Frontend:**
- Next.js 15 (React 19)
- TypeScript
- Tailwind CSS
- Turborepo (builds & caching)

**Backend:**
- Rust API (Axum framework)
- Clean Architecture (4 layers)
- OpenAPI documentation

**Tooling:**
- Meta orchestrator (custom Rust CLI)
- Bacon (Rust hot-reload)
- Turborepo (TypeScript builds)
- Bun (package manager & runtime)

### Project Structure

```
meta-monorepo/
├── apps/
│   ├── api/              # 🦀 Rust API (Axum + Clean Architecture)
│   ├── web/              # ⚛️ Next.js marketing site
│   └── app/              # ⚛️ Next.js application
│
├── packages/             # 8 shared packages
│   ├── ui/               # React components
│   ├── supabase/         # Database client
│   ├── analytics/        # PostHog integration
│   └── ...
│
├── tooling/
│   ├── meta/             # 🎯 Meta orchestrator (Rust CLI)
│   ├── typescript/       # Shared TS config
│   └── e2e/              # Playwright tests
│
├── docs/                 # 📚 Documentation
│   ├── meta/             # Meta-specific docs
│   ├── guides/           # Tutorials
│   ├── current/          # Architecture
│   └── releases/         # Version notes
│
├── meta.toml             # Meta configuration
├── turbo.json            # Turborepo config
├── CHANGELOG.md          # Version history
└── README.md             # Project overview
```

---

## 🎯 Common Tasks

### Development Workflows

**Start everything:**
```bash
meta dev              # All projects
meta tui              # TUI with live logs

<meta
 dev -p api -p web  # Specific projects
```

**Build:**
```bash
meta build            # Development build
meta build --prod     # Production build
```

**Test:**
```bash
meta test             # All tests
```

**Documentation:**
- [Development Workflows](guides/EXAMPLES.md#development-workflows)
- [Building Projects](guides/EXAMPLES.md#building-projects)
- [Testing](guides/EXAMPLES.md#testing)

### Meta TUI Features

**Launch:**
```bash
meta tui
```

**Keyboard Shortcuts:**
| Key | Action |
|-----|--------|
| `↑/k`, `↓/j` | Navigate projects |
| `Enter` | Toggle log filter |
| `a` | Show all logs |
| `c` | Clear log buffer |
| `q` | Quit |

**Features:**
- Real-time log streaming from all processes
- Color-coded output (white=info, red=errors)
- 1000-line log buffer
- Project status indicators
- Timestamps on all logs

**Documentation:** [meta/LOG_STREAMING.md](meta/LOG_STREAMING.md)

### Configuration

**meta.toml structure:**
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

**Full example:** `meta.toml` in project root

---

## 🔍 Finding Documentation

### By Topic
- **Meta Orchestrator**: `/docs/meta/`
- **Architecture**: `/docs/current/`
- **How-To Guides**: `/docs/guides/`
- **Release Notes**: `/docs/releases/`
- **Historical**: `/docs/archive/`

### By Component
- **Rust API**: `/apps/api/README.md`
- **Next.js Apps**: `/apps/web/` and `/apps/app/`
- **Shared Packages**: `/packages/*/`
- **Meta CLI**: `/tooling/meta/README.md`

### Quick Search
```bash
# Search all docs
grep -r "topic" docs/

# Search meta docs
grep -r "log streaming" docs/meta/

# Search guides
grep -r "example" docs/guides/
```

---

## 📖 API Reference

### Meta CLI

```bash
meta <COMMAND>

Commands:
  init   - Initialize meta.toml configuration
  dev    - Start development servers
  build  - Build all projects
  test   - Run tests
  tui    - Interactive TUI mode (with log streaming)
  help   - Print help information

Options:
  -p, --projects <NAMES>  - Run specific projects only
  --prod                  - Production mode (for build)
  -h, --help              - Print help
  -V, --version           - Print version
```

**Examples:**
```bash
# Start all dev servers
meta dev

# Start specific projects
meta dev -p api -p web

# Production build
meta build --prod

# Interactive TUI
meta tui
```

**Documentation:** [meta/README.md#cli-commands](meta/README.md#cli-commands)

---

## 🏷️ Documentation Conventions

### Status Markers
- ✅ **CURRENT** - Active, up-to-date documentation
- 📝 **DRAFT** - Work in progress
- 📦 **ARCHIVED** - Historical, kept for reference

### Timestamps
Archived documentation uses format: `YYYY-MM-DD_FILENAME.md` or stored in `archive/`

### Linking
Always use relative paths:
```markdown
[Meta Guide](./meta/README.md)
[Examples](./guides/EXAMPLES.md)
```

---

## 📊 Project Status

### Current Version
- **Meta Orchestrator:** v0.2.0
- **Monorepo:** rust-v1
- **Last Updated:** 2025-01-19

### Build Status
- ✅ Meta CLI - Compiles successfully (2.7 MB binary)
- ✅ Rust API - All tests passing (5/5)
- ✅ TypeScript - Type checking passes
- ✅ CI/CD - All workflows green

### Feature Status

| Feature | Status | Docs |
|---------|--------|------|
| Meta CLI | ✅ v0.2.0 | [meta/README.md](meta/README.md) |
| TUI Dashboard | ✅ v0.2.0 | [meta/README.md](meta/README.md) |
| Log Streaming | ✅ v0.2.0 | [meta/LOG_STREAMING.md](meta/LOG_STREAMING.md) |
| Log Filtering | ✅ v0.2.0 | [guides/EXAMPLES.md](guides/EXAMPLES.md) |
| Rust API | ✅ Production | [../apps/api/README.md](../apps/api/README.md) |
| Watch Mode | 🚧 v0.3.0 | Planned |
| Log Search | 🚧 v0.3.0 | Planned |

---

## 🔗 External Resources

### Tool Documentation
- **Meta:** [meta/README.md](meta/README.md)
- **Bacon:** https://github.com/Canop/bacon
- **Turborepo:** https://turbo.build/repo/docs
- **Ratatui:** https://ratatui.rs
- **Axum:** https://docs.rs/axum
- **Next.js:** https://nextjs.org/docs

### Inspiration
- **Clean Architecture** - Separation of concerns, domain-driven design
- **T3 Stack** - TypeScript best practices
- **Vercel** - Deployment and monorepo best practices

---

## 🤝 Contributing

### Documentation Guidelines

1. **Keep docs up to date** - Update when making changes
2. **Use clear examples** - Show, don't just tell
3. **Link between docs** - Cross-reference related content
4. **Follow structure** - Place docs in correct subdirectory
5. **Add to index** - Update this INDEX.md when adding docs

### Adding Documentation

**New guide:**
```bash
# Create in guides/
touch docs/guides/YOUR_GUIDE.md

# Add link to INDEX.md
# Test all relative links
```

**New release notes:**
```bash
# Create in releases/
touch docs/releases/RELEASE_vX.Y.Z.md

# Update CHANGELOG.md
# Update INDEX.md
```

**Archive old docs:**
```bash
# Move to archive/
mv docs/OLD_DOC.md docs/archive/

# Update INDEX.md
# Add deprecation notice if needed
```

---

## 📋 Checklists

### New Developer Onboarding

- [ ] Clone repository
- [ ] Run `bun install`
- [ ] Install meta: `cd tooling/meta && ./install.sh`
- [ ] Verify: `meta --version` shows 0.2.0
- [ ] Configure environment files (`.env`)
- [ ] Run `meta tui` to start everything
- [ ] Read [Getting Started Guide](guides/GETTING_STARTED.md)
- [ ] Try examples from [EXAMPLES.md](guides/EXAMPLES.md)

### Release Checklist

- [ ] Update version in `tooling/meta/Cargo.toml`
- [ ] Update `CHANGELOG.md` with release notes
- [ ] Create `docs/releases/RELEASE_vX.Y.Z.md`
- [ ] Update feature documentation
- [ ] Test: `meta build --prod`
- [ ] Test: `cargo test` in all Rust projects
- [ ] Update `docs/INDEX.md` (this file)
- [ ] Tag release in git
- [ ] Update root `README.md` if needed

---

## 🆘 Getting Help

### Quick Links

- **Installation issues:** [guides/EXAMPLES.md#common-issues](guides/EXAMPLES.md#common-issues)
- **Meta configuration:** [meta/README.md#configuration](meta/README.md#configuration)
- **Usage examples:** [guides/EXAMPLES.md](guides/EXAMPLES.md)
- **Architecture questions:** [current/ARCHITECTURE.md](current/ARCHITECTURE.md)
- **API docs:** [../apps/api/README.md](../apps/api/README.md)

### Common Issues

**Meta not found:**
```bash
cd tooling/meta && ./install.sh
export PATH="$HOME/.cargo/bin:$PATH"
```

**Port already in use:**
```bash
lsof -ti:4400 | xargs kill -9
# Or change PORT in .env files
```

**Missing tools:**
```bash
cargo install bacon    # Rust hot-reload
npm install -g turbo   # TypeScript builds
```

---

## 📝 License

MIT License - See [LICENSE.md](../LICENSE.md)

---

**Navigation**: [Root](../README.md) | [Meta](meta/) | [Guides](guides/) | [Current Docs](current/) | [Releases](releases/) | [Archive](archive/)

**Happy coding with Meta! 🚀**

*Last updated: 2025-01-19 by Meta Team*
