# Monorepo Structure

**Status**: ✅ CURRENT
**Last Updated**: 2025-11-18

---

## Overview

Fullstack V1 uses Turborepo to manage a monorepo with multiple apps and shared packages. This structure promotes code reuse, consistent tooling, and efficient builds.

## Directory Structure

```
fullstack-v1/
├── apps/                      # Deployable applications
│   ├── app/                   # Main SaaS application
│   ├── web/                   # Marketing website
│   └── api/                   # Supabase local setup
├── packages/                  # Shared packages
│   ├── react-query/           # TanStack Query setup
│   ├── supabase/              # Supabase clients
│   ├── ui/                    # UI components
│   ├── analytics/             # PostHog integration
│   ├── email/                 # Email templates
│   ├── jobs/                  # Background jobs
│   ├── kv/                    # Redis utilities
│   └── logger/                # Logging utilities
├── tooling/                   # Development tools
│   ├── typescript/            # TS configs
│   └── e2e/                   # Playwright tests
├── docs/                      # Documentation
├── biome.json                 # Linter/formatter config
├── turbo.json                 # Turborepo config
├── package.json               # Root package.json
└── bun.lock                   # Lock file
```

## Apps

### apps/app
**Purpose**: Main SaaS application
**Tech**: Next.js 14, React Query, Supabase
**Port**: 4402

**Structure**:
```
apps/app/
├── src/
│   ├── app/                  # Next.js app router
│   │   ├── [locale]/        # Internationalization
│   │   ├── api/             # API routes
│   │   └── ...
│   ├── actions/             # Server actions
│   ├── components/          # React components
│   ├── locales/             # i18n translations
│   └── middleware.ts        # Auth middleware
├── public/                   # Static assets
├── package.json
├── next.config.mjs
├── tailwind.config.ts
└── tsconfig.json
```

**Dependencies**:
- `@v1/react-query`
- `@v1/supabase`
- `@v1/ui`
- `@v1/analytics`
- `@v1/kv`

### apps/web
**Purpose**: Marketing website
**Tech**: Next.js 14, Tailwind CSS
**Port**: 4401

**Structure**:
```
apps/web/
├── src/
│   ├── app/                  # Next.js app router
│   ├── components/          # React components
│   └── ...
├── public/                   # Static assets
├── package.json
└── ...
```

**Dependencies**:
- `@v1/ui`
- `@v1/analytics`

### apps/api
**Purpose**: Local Supabase development
**Tech**: Supabase CLI, Docker

**Structure**:
```
apps/api/
├── supabase/
│   ├── migrations/          # Database migrations
│   ├── functions/           # Edge functions
│   └── config.toml          # Supabase config
└── package.json
```

## Packages

### packages/react-query
**Purpose**: TanStack Query configuration and hooks

**Exports**:
```typescript
// Provider
import { ReactQueryProvider } from '@v1/react-query'

// Hooks
import { useServerQuery } from '@v1/react-query'
import { useServerMutation } from '@v1/react-query'
import { useInfiniteServerQuery } from '@v1/react-query'

// Client
import { queryClient } from '@v1/react-query/client'
```

**Structure**:
```
packages/react-query/
├── src/
│   ├── providers/
│   │   └── query-provider.tsx
│   ├── hooks/
│   │   ├── use-server-query.ts
│   │   ├── use-server-mutation.ts
│   │   └── use-infinite-server-query.ts
│   ├── client.ts
│   └── index.ts
└── package.json
```

### packages/supabase
**Purpose**: Supabase clients, queries, and mutations

**Exports**:
```typescript
// Clients
import { createClient } from '@v1/supabase/server'
import { createClient } from '@v1/supabase/client'

// Queries
import { getUser, getPosts } from '@v1/supabase/queries'

// Mutations
import { updateUser } from '@v1/supabase/mutations'

// Types
import type { Database, Tables } from '@v1/supabase/types'
```

**Structure**:
```
packages/supabase/
├── src/
│   ├── clients/
│   │   ├── server.ts
│   │   ├── client.ts
│   │   └── middleware.ts
│   ├── queries/
│   │   └── index.ts
│   ├── mutations/
│   │   └── index.ts
│   └── types/
│       ├── index.ts
│       └── db.ts
└── package.json
```

### packages/ui
**Purpose**: Shared UI components

**Exports**:
```typescript
import { Button, Input } from '@v1/ui'
import { cn } from '@v1/ui/cn'
```

**Structure**:
```
packages/ui/
├── src/
│   ├── components/
│   │   ├── ui/              # Shadcn components
│   │   └── ...
│   ├── lib/
│   │   └── utils.ts
│   └── globals.css
└── package.json
```

### packages/analytics
**Purpose**: PostHog analytics integration

### packages/email
**Purpose**: Email templates with React Email

### packages/jobs
**Purpose**: Background jobs with Trigger.dev

### packages/kv
**Purpose**: Redis caching with Upstash

### packages/logger
**Purpose**: Logging utilities

## Tooling

### tooling/typescript
**Purpose**: Shared TypeScript configurations

**Configs**:
- `base.json` - Base config
- `nextjs.json` - Next.js config
- `react-library.json` - React library config

### tooling/e2e
**Purpose**: Playwright end-to-end tests

**Structure**:
```
tooling/e2e/
├── tests/
│   ├── app.spec.ts
│   ├── web.spec.ts
│   └── api.spec.ts
├── playwright.config.ts
└── package.json
```

## Turborepo Configuration

### turbo.json
```json
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": [".next/**", "dist/**"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "lint": {},
    "typecheck": {
      "dependsOn": ["^build"]
    }
  }
}
```

### Build Pipeline
1. Install dependencies (`bun install`)
2. Build packages in dependency order
3. Build apps
4. Run tests/lint/typecheck

## Package Dependencies

### Internal Dependencies
Apps depend on packages:
```
apps/app → @v1/react-query, @v1/supabase, @v1/ui, @v1/analytics
apps/web → @v1/ui, @v1/analytics
```

Packages can depend on other packages:
```
@v1/supabase → @v1/logger
```

### Workspace Protocol
Use `workspace:*` in package.json:
```json
{
  "dependencies": {
    "@v1/react-query": "workspace:*"
  }
}
```

## Scripts

### Root Scripts
```bash
bun dev              # Start all in dev mode
bun dev:app          # Start app only
bun dev:web          # Start web only
bun build            # Build all
bun lint             # Lint all
bun typecheck        # Type check all
bun format           # Format all
bun clean            # Clean node_modules
bun clean:workspaces # Clean build outputs
```

### Package Scripts
Each package has:
- `typecheck` - Type checking
- `lint` - Linting
- `format` - Formatting
- `clean` - Cleanup

## Adding New Packages

### 1. Create Package Directory
```bash
mkdir -p packages/my-package/src
cd packages/my-package
```

### 2. Create package.json
```json
{
  "name": "@v1/my-package",
  "version": "1.0.0",
  "private": true,
  "scripts": {
    "typecheck": "tsc --noEmit",
    "lint": "biome check .",
    "format": "biome format --write ."
  },
  "exports": {
    ".": "./src/index.ts"
  }
}
```

### 3. Create Source Files
```typescript
// src/index.ts
export const myFunction = () => {
  // Implementation
}
```

### 4. Create tsconfig.json
```json
{
  "extends": "@v1/typescript/base.json",
  "compilerOptions": {
    "outDir": "dist",
    "rootDir": "src"
  },
  "include": ["src"]
}
```

### 5. Install in Apps
```bash
cd apps/app
bun add @v1/my-package@workspace:*
```

## Adding New Apps

### 1. Create App Directory
```bash
mkdir -p apps/my-app
cd apps/my-app
```

### 2. Initialize Next.js
```bash
bunx create-next-app@latest . --typescript --tailwind --app
```

### 3. Update package.json
```json
{
  "name": "@v1/my-app",
  "scripts": {
    "dev": "next dev -p 3002",
    "build": "next build",
    "start": "next start"
  },
  "dependencies": {
    "@v1/ui": "workspace:*"
  }
}
```

### 4. Add to Root Scripts
```json
// root package.json
{
  "scripts": {
    "dev:my-app": "turbo dev --filter=@v1/my-app"
  }
}
```

## Best Practices

### 1. Package Naming
- All packages prefixed with `@v1/`
- Use kebab-case: `@v1/my-package`

### 2. Exports
- Use explicit exports in package.json
- Provide type definitions
- Export only public API

### 3. Dependencies
- Keep package dependencies minimal
- Use peer dependencies for shared libraries
- Use workspace protocol for internal deps

### 4. TypeScript
- Extend shared configs
- Enable strict mode
- Generate types for exports

### 5. Documentation
- README in each package
- JSDoc for public APIs
- Examples in docs

---

**Related Documentation**:
- [Architecture](./ARCHITECTURE.md)
- [Tech Stack](./TECH_STACK.md)
- [Getting Started](../guides/GETTING_STARTED.md)
