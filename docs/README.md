# Fullstack V1 Documentation

**Last Updated**: 2025-11-18

This directory contains technical documentation for the Fullstack V1 project.

## Quick Links

- 📋 **[INDEX.md](./INDEX.md)** - Complete documentation index
- 📝 **[Root README](../README.md)** - Project overview and getting started
- 🏗️ **[Architecture](./current/ARCHITECTURE.md)** - System architecture overview
- 🚀 **[Getting Started](./guides/GETTING_STARTED.md)** - Quick start guide
- 🤖 **[MCP Servers](./MCP.md)** - Claude Code MCP setup and usage

## Documentation Structure

### 🏗️ Current (`current/`)
Active, up-to-date documentation:
- **[ARCHITECTURE.md](./current/ARCHITECTURE.md)** - System architecture and design
- **[MONOREPO_STRUCTURE.md](./current/MONOREPO_STRUCTURE.md)** - Monorepo organization
- **[TECH_STACK.md](./current/TECH_STACK.md)** - Technology decisions

### 📖 Guides (`guides/`)
Step-by-step guides and tutorials:
- **[GETTING_STARTED.md](./guides/GETTING_STARTED.md)** - Setup and installation
- **[REACT_QUERY_GUIDE.md](./guides/REACT_QUERY_GUIDE.md)** - Using TanStack Query
- **[SUPABASE_GUIDE.md](./guides/SUPABASE_GUIDE.md)** - Supabase integration
- **[DEPLOYMENT_GUIDE.md](./guides/DEPLOYMENT_GUIDE.md)** - Deploying applications

### 🏛️ Architecture (`architecture/`)
Architecture Decision Records (ADRs) and design documents

### ⚙️ Operations (`operations/`)
Operational guides for deployment, monitoring, and maintenance

### 🗺️ Roadmaps (`roadmaps/`)
Product roadmaps and planning documents

### 🔌 API (`api/`)
API documentation and specifications

### 🛠️ Development (`development/`)
Development guides and standards:
- **[DEVELOPMENT_STANDARDS.md](./development/DEVELOPMENT_STANDARDS.md)** - Coding standards
- **[TESTING_GUIDE.md](./development/TESTING_GUIDE.md)** - Testing strategies
- **[CONTRIBUTING.md](./development/CONTRIBUTING.md)** - How to contribute

### 🗄️ Archive (`archive/`)
Historical and deprecated documentation (timestamped)

## Core Concepts

### Tech Stack
- **Frontend**: Next.js 14+ with TypeScript
- **State Management**: TanStack Query (React Query)
- **Backend**: Supabase (PostgreSQL, Auth, Storage)
- **UI**: TailwindCSS + Shadcn UI
- **Monorepo**: Turborepo + Bun
- **Deployment**: Vercel (apps) + Supabase (backend)
- **Analytics**: PostHog
- **Error Tracking**: Sentry
- **Email**: Resend + React Email
- **Background Jobs**: Trigger.dev

### Key Apps
- **apps/app** - Main SaaS application
  - Authentication with Supabase
  - Dashboard and user management
  - React Query integration
- **apps/web** - Marketing website
  - Landing pages
  - Documentation
- **apps/api** - Supabase local development

### Key Packages
- **packages/react-query** - TanStack Query setup and hooks
- **packages/supabase** - Supabase clients, queries, mutations
- **packages/ui** - Shared UI components (Shadcn)
- **packages/analytics** - PostHog integration
- **packages/email** - Email templates
- **packages/jobs** - Background jobs
- **packages/kv** - Redis caching
- **packages/logger** - Logging utilities

### Development Workflow
```bash
# Install dependencies
bun install

# Start all apps
bun dev

# Start specific app
bun dev:app      # Main app
bun dev:web      # Marketing site

# Build
bun build

# Type check
bun typecheck

# Lint
bun lint

# Format
bun format
```

## Production Deployment

### Vercel
Both Next.js apps are designed for Vercel deployment:
- **apps/app** - Main application
- **apps/web** - Marketing site

### Supabase
Backend services through Supabase:
- PostgreSQL database
- Authentication
- Storage
- Edge Functions

### Environment Variables
Required environment variables:
- Supabase credentials
- PostHog API keys
- Sentry DSN
- Resend API key
- Trigger.dev credentials
- Upstash Redis URL

See `.env.example` files in each app.

## Contributing

1. Read [CONTRIBUTING.md](./development/CONTRIBUTING.md)
2. Review [DEVELOPMENT_STANDARDS.md](./development/DEVELOPMENT_STANDARDS.md)
3. Follow the monorepo structure in [MONOREPO_STRUCTURE.md](./current/MONOREPO_STRUCTURE.md)
4. Write tests following [TESTING_GUIDE.md](./development/TESTING_GUIDE.md)
5. All database operations go through Supabase package
6. Use React Query hooks for data fetching

## Development Standards

- **Definition of Done**: Build, tests, and lint must pass
- **Definition of Ready**: Tasks must be clearly defined and testable
- See [DEVELOPMENT_STANDARDS.md](./development/DEVELOPMENT_STANDARDS.md) for details

## Getting Help

- Check the [INDEX.md](./INDEX.md) for complete documentation index
- Review specific guides in `guides/`
- Check app-specific README files in `apps/`
- Check package-specific README files in `packages/`
- Create an issue with `[docs]` prefix for documentation questions

## Directory Overview

```
docs/
├── README.md                    # This file
├── INDEX.md                     # Complete documentation index
├── current/                     # Active documentation
│   ├── ARCHITECTURE.md
│   ├── MONOREPO_STRUCTURE.md
│   └── TECH_STACK.md
├── guides/                      # How-to guides
│   ├── GETTING_STARTED.md
│   ├── REACT_QUERY_GUIDE.md
│   ├── SUPABASE_GUIDE.md
│   └── DEPLOYMENT_GUIDE.md
├── architecture/                # ADRs and design docs
├── operations/                  # Operational guides
├── roadmaps/                    # Planning documents
├── api/                         # API documentation
├── development/                 # Development guides
│   ├── DEVELOPMENT_STANDARDS.md
│   ├── TESTING_GUIDE.md
│   └── CONTRIBUTING.md
├── archive/                     # Historical docs
└── images/                      # Images and diagrams
    └── logo.svg
```

For project overview, see the main [README.md](../README.md) at the project root.

---

**Navigation**: [Home](../README.md) | [Index](./INDEX.md) | [Architecture](./current/) | [Guides](./guides/) | [Archive](./archive/)
