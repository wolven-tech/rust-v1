# Getting Started with rust-v1

**Status**: ✅ CURRENT
**Last Updated**: 2025-01-19

---

## Prerequisites

Before you begin, ensure you have the following installed:

- **[Bun](https://bun.sh/)** v1.1.26+ - JavaScript runtime and package manager
- **[Rust](https://rustup.rs/)** v1.70+ - Systems programming language
- **[Bacon](https://github.com/Canop/bacon)** - Rust hot-reload (`cargo install bacon`)
- **[Docker](https://www.docker.com/)** - For local Supabase (optional)
- **[Git](https://git-scm.com/)** - Version control

## Quick Start

### 1. Create Your Project

**Recommended:** Use `degit` to scaffold from this template:

```bash
# Create a new project (without git history)
bunx degit wolven-tech/rust-v1 my-project
cd my-project
```

**Alternative:** Clone the repository directly:

```bash
git clone https://github.com/wolven-tech/rust-v1.git
cd rust-v1
```

### 2. Install Dependencies

```bash
bun install
```

This will install all dependencies for all apps and packages in the monorepo.

### 3. Install Meta Orchestrator

```bash
cd tooling/meta && ./install.sh && cd ../..
```

This installs the meta CLI tool globally, giving you access to the unified task runner.

### 4. Set Up Environment Variables

Copy the example environment files:

```bash
# Main app
cp apps/app/.env.example apps/app/.env.local

# Marketing site
cp apps/web/.env.example apps/web/.env.local

# Supabase
cp apps/api/.env.example apps/api/.env
```

Edit the `.env.local` files with your actual credentials (see [Environment Variables](#environment-variables) below).

### 5. Start Development

```bash
# Use meta TUI for unified development experience
meta tui

# Or start all apps without TUI
meta dev

# Or start specific apps
bun dev

# Or start individual apps
bun dev:app    # Main app on localhost:4402
bun dev:web    # Marketing on localhost:4401
```

### 5. Open in Browser

- **Main App**: http://localhost:4402
- **Marketing Site**: http://localhost:4401

## Environment Variables

### Required Services

You'll need accounts and API keys for:

1. **[Supabase](https://supabase.com/)**
   - Create a new project
   - Get your project URL and anon key

2. **[PostHog](https://posthog.com/)**
   - Sign up for free account
   - Get your project API key

3. **[Resend](https://resend.com/)**
   - Sign up and verify your domain
   - Get your API key

4. **[Sentry](https://sentry.io/)**
   - Create a new project
   - Get your DSN

5. **[Upstash](https://upstash.com/)**
   - Create a Redis database
   - Get your REST URL and token

6. **[Trigger.dev](https://trigger.dev/)**
   - Create a new project
   - Get your API key

### apps/app/.env.local

```bash
# Supabase
NEXT_PUBLIC_SUPABASE_URL=https://your-project.supabase.co
NEXT_PUBLIC_SUPABASE_ANON_KEY=your-anon-key

# PostHog
NEXT_PUBLIC_POSTHOG_KEY=phc_your_key
NEXT_PUBLIC_POSTHOG_HOST=https://app.posthog.com

# Sentry
NEXT_PUBLIC_SENTRY_DSN=https://xxx@xxx.ingest.sentry.io/xxx
SENTRY_AUTH_TOKEN=your-auth-token

# Upstash Redis
UPSTASH_REDIS_REST_URL=https://your-redis.upstash.io
UPSTASH_REDIS_REST_TOKEN=your-token

# Resend
RESEND_API_KEY=re_your_key

# Trigger.dev
TRIGGER_API_KEY=tr_your_key
TRIGGER_API_URL=https://api.trigger.dev

# Dub
DUB_API_KEY=your-dub-key
```

### apps/web/.env.local

```bash
# PostHog
NEXT_PUBLIC_POSTHOG_KEY=phc_your_key
NEXT_PUBLIC_POSTHOG_HOST=https://app.posthog.com
```

## Project Structure

```
fullstack-v1/
├── apps/
│   ├── app/          # Main SaaS application
│   ├── web/          # Marketing website
│   └── api/          # Supabase setup
├── packages/
│   ├── react-query/  # TanStack Query setup
│   ├── supabase/     # Database layer
│   ├── ui/           # UI components
│   └── ...
└── docs/             # Documentation
```

See [Monorepo Structure](../current/MONOREPO_STRUCTURE.md) for detailed information.

## Development Workflow

### Running Commands

```bash
# Development
bun dev              # Start all apps
bun dev:app          # Start main app only
bun dev:web          # Start marketing site only

# Building
bun build            # Build all apps
bun build --filter=@v1/app  # Build specific app

# Code Quality
bun lint             # Lint all code
bun format           # Format all code
bun typecheck        # Type check all code

# Cleaning
bun clean            # Remove node_modules
bun clean:workspaces # Remove build outputs
```

### Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make your changes**:
   - Edit files in `apps/` or `packages/`
   - Add tests if needed
   - Update documentation

3. **Check your work**:
   ```bash
   bun lint
   bun typecheck
   bun build
   ```

4. **Commit and push**:
   ```bash
   git add .
   git commit -m "feat: add my feature"
   git push origin feature/my-feature
   ```

## Common Tasks

### Adding a New Page

In `apps/app/src/app/`:

```typescript
// apps/app/src/app/my-page/page.tsx
export default function MyPage() {
  return (
    <div>
      <h1>My New Page</h1>
    </div>
  )
}
```

### Creating a Server Action

In `apps/app/src/actions/`:

```typescript
// apps/app/src/actions/my-action.ts
'use server'

import { authActionClient } from '@/actions/safe-action'
import { z } from 'zod'

const schema = z.object({
  name: z.string()
})

export const myAction = authActionClient
  .schema(schema)
  .action(async ({ parsedInput, ctx }) => {
    // Your logic here
    return { success: true }
  })
```

### Using React Query

In your component:

```typescript
'use client'

import { useServerQuery } from '@v1/react-query'
import { getPosts } from '@v1/supabase/queries'

export function PostList() {
  const { data, isLoading } = useServerQuery('posts', getPosts)

  if (isLoading) return <div>Loading...</div>

  return (
    <ul>
      {data?.map((post) => (
        <li key={post.id}>{post.title}</li>
      ))}
    </ul>
  )
}
```

### Adding a UI Component

In `packages/ui/src/components/`:

```typescript
// packages/ui/src/components/my-component.tsx
export function MyComponent() {
  return <div>My Component</div>
}

// packages/ui/src/index.ts
export { MyComponent } from './components/my-component'
```

Use it in your app:

```typescript
import { MyComponent } from '@v1/ui'

export default function Page() {
  return <MyComponent />
}
```

## Database Setup

### Option 1: Use Hosted Supabase (Recommended)

1. Go to [supabase.com](https://supabase.com)
2. Create a new project
3. Copy your project URL and anon key
4. Add them to `.env.local`

### Option 2: Run Supabase Locally

```bash
cd apps/api
supabase start
```

This will start:
- PostgreSQL on port 54322
- Studio on http://localhost:54323
- API on http://localhost:54321

## Testing

### End-to-End Tests

```bash
cd tooling/e2e
bun test
```

### Run Specific Tests

```bash
bun test:app    # Test main app
bun test:web    # Test marketing site
```

## Troubleshooting

### Port Already in Use

If you get "port already in use" errors:

```bash
# Find and kill process on port 3000
lsof -ti:4402 | xargs kill -9
```

### TypeScript Errors

```bash
# Clean and rebuild
bun clean:workspaces
bun install
bun typecheck
```

### Supabase Connection Issues

1. Check your `.env.local` has correct values
2. Verify your Supabase project is running
3. Check network connectivity
4. Review Supabase dashboard for errors

### React Query Not Working

1. Ensure `ReactQueryProvider` is in your layout
2. Check server actions are marked with `'use server'`
3. Verify query keys are unique
4. Check browser console for errors

## Next Steps

Now that you're set up:

1. **[Architecture](../current/ARCHITECTURE.md)** - Understand the system design
2. **[React Query Guide](./REACT_QUERY_GUIDE.md)** - Learn data fetching patterns
3. **[Supabase Guide](./SUPABASE_GUIDE.md)** - Database and auth
4. **[Deployment Guide](./DEPLOYMENT_GUIDE.md)** - Deploy to production

## Getting Help

- 📖 Check the [Documentation](../README.md)
- 🐛 [Create an issue](https://github.com/your-org/fullstack-v1/issues)
- 💬 Join our community (if applicable)

---

**Related Documentation**:
- [Architecture](../current/ARCHITECTURE.md)
- [Monorepo Structure](../current/MONOREPO_STRUCTURE.md)
- [Development Standards](../development/DEVELOPMENT_STANDARDS.md)
