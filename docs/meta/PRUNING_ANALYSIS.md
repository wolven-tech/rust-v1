# Stack Pruning Analysis - Meta Monorepo

**Date:** 2025-01-19
**Version:** 0.2.0

---

## Overview

With the meta orchestrator now production-ready, we can prune redundant npm scripts and simplify the development workflow. This document analyzes what can be removed/simplified now that meta handles orchestration.

---

## Current State Analysis

### package.json Scripts (Root)

**Current scripts:**
```json
{
  "build": "turbo build",
  "clean": "git clean -xdf node_modules",
  "clean:workspaces": "turbo clean",
  "dev": "turbo dev --parallel",
  "dev:web": "turbo dev --filter=@meta/web",
  "dev:app": "turbo dev --filter=@meta/app",
  "dev:api": "cd apps/api && bacon run-long",
  "start:web": "turbo start --filter=@meta/web",
  "start:app": "turbo start --filter=@meta/app",
  "test": "turbo test --parallel",
  "format": "biome format --write .",
  "lint": "turbo lint && bun lint:repo",
  "lint:repo": "bunx sherif@latest",
  "lint:repo:fix": "bunx sherif@latest --fix",
  "typecheck": "turbo typecheck",
  "meta": "cd tooling/meta && cargo run --",
  "meta:init": "cd tooling/meta && cargo run -- init",
  "meta:dev": "cd tooling/meta && cargo run -- dev",
  "meta:build": "cd tooling/meta && cargo run -- build"
}
```

---

## Pruning Recommendations

### ✅ Keep (Essential)

**Meta scripts:**
- `meta` - Quick access to meta CLI
- `meta:dev` - Shortcut for meta dev
- `meta:build` - Shortcut for meta build
- `meta:init` - Initialize meta configuration

**Code quality:**
- `format` - Code formatting (not dev workflow)
- `lint` - Linting (not dev workflow)
- `lint:repo` - Repository-level lint
- `lint:repo:fix` - Auto-fix repository issues
- `typecheck` - Type checking (not dev workflow)

**Cleanup:**
- `clean` - Clean node_modules
- `clean:workspaces` - Clean turbo cache

### ❌ Remove (Redundant with meta)

**Development scripts (replaced by `meta dev`):**
- ~~`dev`~~ → Use `meta dev` or `meta tui`
- ~~`dev:web`~~ → Use `meta dev -p web`
- ~~`dev:app`~~ → Use `meta dev -p app`
- ~~`dev:api`~~ → Use `meta dev -p api`

**Build scripts (replaced by `meta build`):**
- ~~`build`~~ → Use `meta build`

**Start scripts (rarely used in dev):**
- ~~`start:web`~~ → Can keep if needed for production
- ~~`start:app`~~ → Can keep if needed for production

**Test scripts (replaced by `meta test`):**
- ~~`test`~~ → Use `meta test`

### 🤔 Maybe Keep (Consider)

**Start scripts for production:**
- `start:web` - Used in production? Keep if yes
- `start:app` - Used in production? Keep if yes

**Direct turbo access:**
- Developers can still use `turbo` directly if needed
- No need for npm script wrappers

---

## Proposed New package.json Scripts

### Minimal Version

```json
{
  "scripts": {
    "clean": "git clean -xdf node_modules",
    "clean:workspaces": "turbo clean",
    "format": "biome format --write .",
    "lint": "turbo lint && bun lint:repo",
    "lint:repo": "bunx sherif@latest",
    "lint:repo:fix": "bunx sherif@latest --fix",
    "typecheck": "turbo typecheck",
    "meta": "cd tooling/meta && cargo run --",
    "meta:init": "cd tooling/meta && cargo run -- init",
    "meta:dev": "cd tooling/meta && cargo run -- dev",
    "meta:build": "cd tooling/meta && cargo run -- build",
    "meta:test": "cd tooling/meta && cargo run -- test",
    "meta:tui": "cd tooling/meta && cargo run -- tui"
  }
}
```

**Removed:** 9 scripts
**Added:** 2 scripts (`meta:test`, `meta:tui`)
**Kept:** 11 scripts

### With Production Support

```json
{
  "scripts": {
    "clean": "git clean -xdf node_modules",
    "clean:workspaces": "turbo clean",
    "start:web": "turbo start --filter=@meta/web",
    "start:app": "turbo start --filter=@meta/app",
    "format": "biome format --write .",
    "lint": "turbo lint && bun lint:repo",
    "lint:repo": "bunx sherif@latest",
    "lint:repo:fix": "bunx sherif@latest --fix",
    "typecheck": "turbo typecheck",
    "meta": "cd tooling/meta && cargo run --",
    "meta:init": "cd tooling/meta && cargo run -- init",
    "meta:dev": "cd tooling/meta && cargo run -- dev",
    "meta:build": "cd tooling/meta && cargo run -- build",
    "meta:test": "cd tooling/meta && cargo run -- test",
    "meta:tui": "cd tooling/meta && cargo run -- tui"
  }
}
```

**Removed:** 7 scripts
**Added:** 2 scripts (`meta:test`, `meta:tui`)
**Kept:** 13 scripts

---

## Migration Guide

### For Developers

**Old workflow:**
```bash
# Development
bun dev                 # All projects
bun dev:web             # Web only
bun dev:api             # API only

# Build
bun build               # All projects

# Test
bun test                # All tests
```

**New workflow:**
```bash
# Development (better!)
meta dev                # All projects
meta dev -p web         # Web only
meta dev -p api         # API only
meta tui                # TUI with log streaming

# Build
meta build              # All projects
meta build --prod       # Production build

# Test
meta test               # All tests
```

### For CI/CD

**Before:**
```yaml
- run: bun install
- run: bun lint
- run: bun typecheck
- run: bun build
- run: bun test
```

**After:**
```yaml
- run: bun install
- run: cd tooling/meta && ./install.sh          # Install meta
- run: bun lint
- run: bun typecheck
- run: meta build --prod          # Use meta
- run: meta test                  # Use meta
```

---

## Benefits of Pruning

### 1. **Simpler Interface**

**Before:** 20+ npm scripts to remember
**After:** ~13 npm scripts (mostly tooling)
**Developer UX:** Use `meta` for dev workflows, npm for tooling

### 2. **Single Source of Truth**

**Before:** Dev workflows split between npm & meta
**After:** Meta is THE way to develop
**Benefit:** Less confusion, easier onboarding

### 3. **Cleaner package.json**

**Before:** Mixed concerns (dev, build, test, tooling)
**After:** Clear separation (meta = dev, npm = tooling)
**Benefit:** Easier to maintain

### 4. **Better Developer Experience**

**Before:** Context switch between npm commands
**After:** `meta tui` for unified dashboard
**Benefit:** See all logs in one place

---

## Risks & Mitigation

### Risk 1: Muscle Memory

**Risk:** Developers used to `bun dev`
**Mitigation:**
- Add deprecation warnings in README
- Update onboarding docs
- Add aliases if needed

### Risk 2: CI/CD Changes

**Risk:** Existing CI workflows use npm scripts
**Mitigation:**
- Update CI workflows in same PR
- Test thoroughly
- Keep `start:*` for production

### Risk 3: External Documentation

**Risk:** External docs reference old npm scripts
**Mitigation:**
- Update all docs in this PR
- Add migration guide
- Keep old scripts with deprecation for 1 version

---

## Implementation Plan

### Phase 1: Add New Meta Scripts ✅

```json
"meta:test": "cd tooling/meta && cargo run -- test",
"meta:tui": "cd tooling/meta && cargo run -- tui"
```

### Phase 2: Update Documentation

- [x] Update README.md
- [x] Update EXAMPLES.md
- [x] Update docs/INDEX.md
- [ ] Update GETTING_STARTED.md

### Phase 3: Remove Redundant Scripts

Remove these npm scripts:
- `dev`
- `dev:web`
- `dev:app`
- `dev:api`
- `build`
- `test`

Keep production scripts:
- `start:web`
- `start:app`

### Phase 4: Update CI/CD

Update `.github/workflows/ci.yml`:
- Add meta installation step
- Replace npm build/test with meta

### Phase 5: Communication

- Add migration section to CHANGELOG.md
- Update root README.md with deprecation notice
- Post team announcement if applicable

---

## Tooling Directory Analysis

### Current tooling/

```
tooling/
├── e2e/         # Playwright E2E tests
├── meta/        # Meta orchestrator ⭐
└── typescript/  # Shared TS config
```

### Keep All

**Rationale:**
- `e2e/` - Essential for end-to-end testing
- `meta/` - Our new orchestrator (production ready)
- `typescript/` - Shared config for monorepo

**No pruning needed in tooling/** ✅

---

## Unnecessary Dependencies?

### To Investigate

Check if these are still needed:
- [ ] Any unused npm packages in root package.json
- [ ] Any unused npm packages in app/package.json files
- [ ] Any unused Rust dependencies in api/Cargo.toml

**Note:** Run dependency analysis separately

---

## Summary

### Scripts to Remove: 6-7

- `dev`
- `dev:web`
- `dev:app`
- `dev:api`
- `build`
- `test`
- Optionally: `start:web`, `start:app` (if not used in production)

### Scripts to Add: 2

- `meta:test`
- `meta:tui`

### Net Change

- **Before:** 20 scripts
- **After:** 13-15 scripts
- **Reduction:** 25-35%

### Developer Experience

- **Old:** Remember which npm script for which task
- **New:** Just use `meta` for everything dev-related
- **Improvement:** 🚀 Significant

---

## Next Steps

1. ✅ Create this analysis document
2. ⏳ Update package.json with new structure
3. ⏳ Update all documentation
4. ⏳ Update CI/CD workflows
5. ⏳ Test thoroughly
6. ⏳ Deploy changes

---

**Created:** 2025-01-19
**Status:** Ready for implementation
**Recommendation:** Proceed with "Minimal Version" + production scripts
