# Fullstack V1 Architecture

**Status**: ✅ CURRENT
**Last Updated**: 2025-11-18
**Authors**: Wolven Tech Advisory

---

## Overview

Fullstack V1 is a production-ready SaaS starter built as a monorepo using modern full-stack technologies. The architecture emphasizes type safety, code reuse, and scalability.

## Architecture Principles

### 1. Monorepo Structure
- **Turborepo** for build orchestration
- **Bun** for package management and runtime
- Shared packages for maximum code reuse
- Independent deployability of apps

### 2. Type Safety
- **TypeScript** throughout the stack
- Shared types between frontend and backend
- Supabase generates types from database schema
- React Query provides type-safe data fetching

### 3. Server State Management
- **TanStack Query** for all server state
- Automatic caching and revalidation
- Optimistic updates
- Background refetching

### 4. Clean Separation of Concerns
- **Apps** - Deployable applications
- **Packages** - Shared libraries
- **Tooling** - Development tools and configs

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                            │
├─────────────────────────────────────────────────────────────────┤
│  apps/app (Next.js)          │  apps/web (Next.js)              │
│  - User Dashboard            │  - Marketing Site                │
│  - Authentication            │  - Documentation                 │
│  - React Query               │  - Landing Pages                 │
└──────────────────┬───────────┴──────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Shared Packages                            │
├─────────────────────────────────────────────────────────────────┤
│  @v1/react-query   │  @v1/supabase    │  @v1/ui                 │
│  @v1/analytics     │  @v1/email       │  @v1/kv                 │
│  @v1/jobs          │  @v1/logger      │                         │
└──────────────────┬──────────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Backend Services                           │
├─────────────────────────────────────────────────────────────────┤
│  Supabase                  │  External Services                 │
│  - PostgreSQL              │  - PostHog (Analytics)             │
│  - Auth                    │  - Sentry (Monitoring)             │
│  - Storage                 │  - Resend (Email)                  │
│  - Edge Functions          │  - Trigger.dev (Jobs)              │
│                            │  - Upstash (Redis)                 │
└─────────────────────────────────────────────────────────────────┘
```

## Application Layer

### apps/app
Main SaaS application with:
- **Next.js 14+** with App Router
- **Server Components** for initial render
- **Client Components** for interactivity
- **React Query** for data fetching
- **Supabase Auth** for authentication
- **TailwindCSS + Shadcn** for UI

**Key Files**:
- `src/app/[locale]/layout.tsx` - Root layout with providers
- `src/actions/` - Server actions
- `src/components/` - React components
- `src/middleware.ts` - Auth middleware

### apps/web
Marketing website with:
- **Next.js 14+** with App Router
- **Server Components** optimized
- **SEO** optimization
- **TailwindCSS + Shadcn** for UI

## Package Layer

### @v1/react-query
TanStack Query configuration and utilities:
- `ReactQueryProvider` - SSR-friendly provider
- `useServerQuery` - Hook for server actions
- `useServerMutation` - Hook for mutations
- `useInfiniteServerQuery` - Hook for pagination

**Usage**:
```typescript
import { useServerQuery } from '@v1/react-query'

const { data, isLoading } = useServerQuery(
  'posts',
  () => getPosts()
)
```

### @v1/supabase
Supabase client and database layer:
- `createClient()` - Server-side client
- `createClient()` - Browser client
- Queries and mutations
- Type-safe database access

**Exports**:
- `/server` - Server-side client
- `/client` - Browser client
- `/queries` - Database queries
- `/mutations` - Database mutations
- `/types` - Database types

### @v1/ui
Shared UI components:
- Shadcn components
- Custom components
- Theme provider
- Utility functions

### @v1/analytics
PostHog integration:
- Event tracking
- Feature flags
- User identification
- A/B testing

### @v1/email
Email templates and sending:
- React Email components
- Resend integration
- Template management

### @v1/jobs
Background job processing:
- Trigger.dev integration
- Job definitions
- Scheduling

### @v1/kv
Redis caching and rate limiting:
- Upstash integration
- Rate limiting utilities
- Cache helpers

### @v1/logger
Logging utilities:
- Structured logging
- Error tracking
- Debug helpers

## Data Flow

### Server-Side Rendering (SSR)
```
1. Request → Next.js Server
2. Server Component fetches data directly
3. HTML rendered with data
4. Sent to client
5. React Query hydrates on client
```

### Client-Side Data Fetching
```
1. Component mounts
2. React Query checks cache
3. If stale/missing, calls server action
4. Server action → Supabase
5. Data cached by React Query
6. Component re-renders
```

### Mutations
```
1. User action triggers mutation
2. useServerMutation calls server action
3. Optimistic update (optional)
4. Server action → Supabase
5. On success, invalidate queries
6. React Query refetches
```

## Authentication Flow

```
1. User clicks "Sign in with Google"
2. Redirects to Supabase Auth
3. Google OAuth flow
4. Callback to /api/auth/callback
5. Middleware checks auth on protected routes
6. Session stored in cookies
7. Server components read session
```

## Deployment Architecture

### Vercel Deployment
- **apps/app** deployed to Vercel
- **apps/web** deployed to Vercel
- Edge functions at the edge
- ISR for static pages

### Supabase
- PostgreSQL hosted by Supabase
- Auth managed by Supabase
- Storage on Supabase CDN
- Edge functions on Supabase

### External Services
- **PostHog** - Hosted analytics
- **Sentry** - Error tracking
- **Resend** - Email delivery
- **Trigger.dev** - Job processing
- **Upstash** - Redis hosting

## Security

### Authentication
- OAuth with Google
- Session-based auth
- HTTP-only cookies
- CSRF protection

### Authorization
- Row Level Security (RLS) in Supabase
- Server-side checks
- Middleware protection

### Data Protection
- Environment variables encrypted
- Secrets in Vercel
- HTTPS everywhere
- Content Security Policy

## Performance

### Caching Strategy
- React Query: 1 minute stale time
- Next.js: ISR for static pages
- Redis: API response caching
- CDN: Static assets

### Optimization
- Server Components by default
- Code splitting automatic
- Image optimization
- Font optimization

## Monitoring

### Error Tracking
- Sentry for error monitoring
- Source maps uploaded
- Release tracking

### Analytics
- PostHog for product analytics
- Custom events
- Feature flags
- Session replay

### Logging
- Structured logging
- Error aggregation
- Performance metrics

## Scalability

### Horizontal Scaling
- Vercel auto-scales
- Supabase connection pooling
- Redis for distributed state

### Database
- PostgreSQL with indexes
- Connection pooling
- Read replicas (future)

### Caching
- Multi-level caching
- CDN for static assets
- React Query client cache
- Redis server cache

## Development

### Local Development
```bash
bun install
bun dev
```

### Type Safety
- TypeScript strict mode
- Supabase generates types
- Shared types in packages
- Type-safe server actions

### Testing
- Playwright for E2E
- Unit tests for utilities
- Integration tests for APIs

## Future Considerations

### Planned Enhancements
- Websockets for real-time features
- Mobile app with React Native
- Microservices for heavy processing
- Multi-tenancy support

### Scalability Improvements
- Database read replicas
- Separate cache cluster
- CDN optimization
- Edge caching

---

**Related Documentation**:
- [Monorepo Structure](./MONOREPO_STRUCTURE.md)
- [Tech Stack](./TECH_STACK.md)
- [React Query Guide](../guides/REACT_QUERY_GUIDE.md)
- [Supabase Guide](../guides/SUPABASE_GUIDE.md)
