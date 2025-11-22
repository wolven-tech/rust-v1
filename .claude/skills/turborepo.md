# Turborepo Skill

## Core Principles

Turborepo is a high-performance build system for JavaScript/TypeScript monorepos. It orchestrates tasks across packages with intelligent caching and parallelization.

## Running Tasks

### Command Syntax

Always use `turbo run <task>` from the **workspace root**:

```bash
# From workspace root
turbo run build
turbo run dev
turbo run test lint typecheck
```

**Never `cd` into individual packages** - turborepo handles package targeting via filters.

### Task Filtering

Use `--filter` to target specific packages by their **package.json name**:

```bash
# Run dev for a specific package
turbo run dev --filter=@rust-v1/web

# Multiple filters (union)
turbo run build --filter=@rust-v1/web --filter=@rust-v1/app

# Include dependencies of a package
turbo run build --filter=web...

# Include dependents of a package
turbo run test --filter=...ui

# Directory-based filter
turbo run lint --filter="./packages/*"
```

### Common Patterns

```bash
# Development: Run dev server for one app
turbo run dev --filter=@rust-v1/web

# Build all apps
turbo run build

# Typecheck everything
turbo run typecheck

# Lint specific packages
turbo run lint --filter=@rust-v1/web --filter=@rust-v1/app
```

## turbo.json Configuration

### Task Definition

```json
{
  "tasks": {
    "build": {
      "dependsOn": ["^build"],          // Run dependencies' build first
      "outputs": [".next/**"],           // What to cache
      "inputs": ["$TURBO_DEFAULT$", ".env"]
    },
    "dev": {
      "persistent": true,                // Long-running process
      "cache": false                     // Don't cache dev servers
    }
  }
}
```

### Key Properties

- **`dependsOn`**: Task dependencies (`^build` = dependencies' build task)
- **`outputs`**: Files/directories to cache
- **`inputs`**: Files that invalidate cache when changed
- **`persistent`**: Task runs continuously (dev servers, watch mode)
- **`cache`**: Whether to cache results (disable for dev/test watch modes)

## Integration Patterns

### With Meta Tool

When configuring meta.toml for turborepo integration:

```toml
[tools.turborepo]
enabled = true
command = "turbo"  # Just "turbo", not full path
for_languages = ["typescript", "javascript"]

[projects.web]
type = "next"
path = "apps/web"  # Path for reference, not execution directory

[projects.web.tasks]
# Meta runs from workspace root, so just use turbo commands directly
dev = { tool = "turborepo", command = "run dev --filter=@rust-v1/web" }
build = { tool = "turborepo", command = "run build --filter=@rust-v1/web" }
```

**Critical**: Turborepo commands are **always run from workspace root**, not from the package directory.

### Package Naming

Always use the **exact package name** from package.json:

```json
// apps/web/package.json
{
  "name": "@rust-v1/web"  // Use THIS in --filter
}
```

## Best Practices

1. **Run from root**: Always execute turbo from workspace root
2. **Use package names**: Filter by package.json name, not directory name
3. **Persistent tasks**: Mark dev/watch tasks as `persistent: true`
4. **Disable cache**: Set `cache: false` for dev/test watch modes
5. **Stream UI**: Use `"ui": "stream"` in turbo.json for better output
6. **Task dependencies**: Use `dependsOn` to ensure correct build order

## Common Mistakes

❌ `cd apps/web && turbo run dev` - Don't cd into packages
✅ `turbo run dev --filter=@rust-v1/web` - Run from root with filter

❌ `turbo dev --filter=web` - Using directory name
✅ `turbo run dev --filter=@rust-v1/web` - Using package name

❌ `turbo run dev` without filter on specific apps
✅ `turbo run dev --filter=@rust-v1/web` - Always filter for app-specific tasks

## Workspace Structure Example

```
monorepo/
├── turbo.json              # Global task configuration
├── package.json            # Workspace root
├── apps/
│   ├── web/
│   │   └── package.json    # "name": "@rust-v1/web"
│   └── app/
│       └── package.json    # "name": "@rust-v1/app"
└── packages/
    ├── ui/
    │   └── package.json    # "name": "@rust-v1/ui"
    └── utils/
        └── package.json    # "name": "@rust-v1/utils"
```

Commands are **always** run from `monorepo/` root.
