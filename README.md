<div align="center">

# rust-v1

**Full-stack monorepo template with Rust + Next.js, powered by the meta orchestrator.**<br>
Unified CLI for all development tasks, real-time log streaming, and parallel execution across a Rust API and React applications.

[![CI](https://github.com/wolven-tech/rust-v1/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/wolven-tech/rust-v1/actions/workflows/ci.yml)
[![Check](https://github.com/wolven-tech/rust-v1/actions/workflows/check.yml/badge.svg?branch=main)](https://github.com/wolven-tech/rust-v1/actions/workflows/check.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE.md)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000000?logo=rust&logoColor=white)](#)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.5+-3178c6?logo=typescript&logoColor=white)](#)
[![Next.js](https://img.shields.io/badge/Next.js-15-000000?logo=nextdotjs&logoColor=white)](#)

[![Built with Rust, TypeScript, React, and Next.js](https://skillicons.dev/icons?i=rust,typescript,react,nextjs)](https://skillicons.dev)

[Docs](docs/INDEX.md) · [Examples](docs/guides/EXAMPLES.md) · [Install](#install) · [What it does](#what-it-does) · [Architecture](#architecture)

</div>

---

**This repo is a template for new Rust + Next.js projects.** [rust-v2](https://github.com/wolven-tech/rust-v2) is its successor; for new projects, use rust-v2 and refer to this repo only for the meta orchestrator (available as a standalone crate on crates.io).

A monorepo is a single repository containing multiple services and applications. This template brings together a Rust API server (using Axum), two Next.js applications (marketing and product), and a unified orchestration CLI (meta) that starts all of them in a Tmux session with color-coded, filterable logs.

| | |
|---|---|
| **Orchestrator** | Meta CLI: start all servers with `meta dev`, watch specific projects, build for production |
| **API server** | Axum (Rust) with Clean Architecture, 4-layer design, OpenAPI documentation |
| **Frontend apps** | Next.js 15 with React 19, Tailwind CSS, TypeScript—both marketing and application UIs |
| **Package manager** | Bun (JavaScript) and Cargo (Rust), with Turborepo build orchestration |
| **Real-time logs** | Color-coded, filterable output from all processes in one terminal window |

## Install

```sh
bun install                           # Install all dependencies
cargo install monorepo-meta           # Install meta orchestrator, or: cargo binstall monorepo-meta
meta dev                              # Start all dev servers in Tmux
```

For existing projects, [docs/meta/STANDALONE.md](docs/meta/STANDALONE.md) shows how to integrate meta independently.

## What it does

| | |
|---|---|
| **Start all servers** | `meta dev` starts the API, marketing site, and product app in a Tmux session with monitored logs |
| **Filter and watch logs** | Real-time color-coded output from each process, filterable by project or severity; each process in its own Tmux pane |
| **Build for production** | `meta build --prod` compiles the Rust API and Next.js apps; zero-warning binaries, ~6 MB API binary and optimized frontend bundles |
| **Run tests** | `meta test` runs all test suites (Rust unit tests, TypeScript checks, Playwright E2E) |
| **Parallel execution** | Bacon watches Rust code and hot-reloads during development; Turborepo parallelizes TypeScript builds and linting |

## Architecture

```
rust-v1/
├── apps/
│   ├── api/              Rust API server (Axum, 4-layer Clean Architecture)
│   ├── web/              Next.js marketing site
│   └── app/              Next.js product application
├── packages/             Shared TypeScript and Rust libraries
│   ├── ui/               React component library (Tailwind)
│   ├── supabase/         Database client wrapper
│   ├── analytics/        PostHog integration
│   └── ...
├── tooling/
│   ├── meta/             Meta orchestrator (standalone crate: monorepo-meta)
│   ├── typescript/       Shared TypeScript config
│   └── e2e/              Playwright end-to-end tests
├── docs/                 Project documentation
├── meta.toml             Meta configuration
└── turbo.json            Turborepo build config
```

## Why not existing tools

Existing monorepo tools handle either the build graph or process orchestration, but not both with zero config:

- **Turborepo** orchestrates builds but leaves you to start dev servers manually or via shell scripts
- **npm/yarn workspaces** and **Bun workspaces** provide package resolution but no build or process management
- **Tmux** needs scripting to coordinate multi-process starts and logging
- **Lerna** focuses on publishing, not development

Meta combines build awareness (Turborepo for TypeScript, Bacon for Rust) with unified process orchestration (Tmux panes, real-time logs, unified CLI) in ~5 MB with zero configuration.

## Limits

- **Meta: early release.** Core features are stable; watch mode for tests is planned for v0.3.0
- **TypeScript apps only** for dev workflows; Bun is used, but the template works with npm and yarn too
- **Rust toolchain required** for the API; if you only need the frontend, `meta dev -p web app` runs only Next.js
- **Tmux or screen required** for `meta dev` (not available in headless environments; use individual `bun dev` and `cargo run` instead)

## Licence

MIT
