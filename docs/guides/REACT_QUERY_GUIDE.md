# React Query Guide

**Status**: ✅ CURRENT
**Last Updated**: 2025-11-18

---

## Overview

Fullstack V1 uses [TanStack Query](https://tanstack.com/query) (React Query) for server state management. The `@v1/react-query` package provides hooks and utilities optimized for Next.js server actions.

## Why React Query?

- **Automatic Caching** - Smart caching with background refetching
- **TypeScript Support** - Full type safety
- **Optimistic Updates** - Update UI before server responds
- **Devtools** - Inspect queries and mutations
- **SSR Support** - Works with Next.js App Router

## Setup

The setup is already done in `apps/app/src/app/[locale]/layout.tsx`:

```typescript
import { ReactQueryProvider } from '@v1/react-query'

export default function RootLayout({ children }) {
  return (
    <html>
      <body>
        <ReactQueryProvider>
          {children}
        </ReactQueryProvider>
      </body>
    </html>
  )
}
```

## Basic Usage

### Fetching Data

Use `useServerQuery` to fetch data from server actions:

```typescript
'use client'

import { useServerQuery } from '@v1/react-query'
import { getPosts } from '@v1/supabase/queries'

export function PostList() {
  const { data, isLoading, error } = useServerQuery(
    'posts',           // Query key
    () => getPosts()   // Server action
  )

  if (isLoading) return <div>Loading...</div>
  if (error) return <div>Error: {error.message}</div>

  return (
    <ul>
      {data?.map((post) => (
        <li key={post.id}>{post.title}</li>
      ))}
    </ul>
  )
}
```

### Mutations

Use `useServerMutation` for creating, updating, or deleting data:

```typescript
'use client'

import { useServerMutation } from '@v1/react-query'
import { updateUser } from '@v1/supabase/mutations'

export function UserProfile() {
  const mutation = useServerMutation(
    updateUser,
    {
      onSuccess: () => {
        console.log('User updated!')
      }
    },
    ['user'] // Queries to invalidate
  )

  const handleUpdate = () => {
    mutation.mutate({
      name: 'New Name'
    })
  }

  return (
    <button
      onClick={handleUpdate}
      disabled={mutation.isPending}
    >
      {mutation.isPending ? 'Updating...' : 'Update Name'}
    </button>
  )
}
```

### Infinite Queries

Use `useInfiniteServerQuery` for pagination:

```typescript
'use client'

import { useInfiniteServerQuery } from '@v1/react-query'
import { getPostsPaginated } from '@v1/supabase/queries'

export function InfinitePostList() {
  const {
    data,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage
  } = useInfiniteServerQuery(
    'posts-infinite',
    (pageParam) => getPostsPaginated(pageParam),
    {
      getNextPageParam: (lastPage, pages) => {
        return lastPage.hasMore ? pages.length : undefined
      },
      initialPageParam: 0
    }
  )

  return (
    <div>
      {data?.pages.map((page) => (
        page.items.map((post) => (
          <div key={post.id}>{post.title}</div>
        ))
      ))}

      <button
        onClick={() => fetchNextPage()}
        disabled={!hasNextPage || isFetchingNextPage}
      >
        Load More
      </button>
    </div>
  )
}
```

## Query Keys

Query keys are used for caching and refetching. They can be:

```typescript
// Simple string
useServerQuery('user', getUser)

// Array with parameters
useServerQuery(['post', postId], () => getPost(postId))

// Complex keys
useServerQuery(['posts', { status: 'published', page: 1 }], getPosts)
```

### Best Practices

1. **Use descriptive keys**:
   ```typescript
   // Good
   useServerQuery(['posts', 'published'], getPublishedPosts)

   // Bad
   useServerQuery('data', getData)
   ```

2. **Include parameters in key**:
   ```typescript
   useServerQuery(['user', userId], () => getUser(userId))
   ```

3. **Hierarchical keys**:
   ```typescript
   ['posts']                    // All posts
   ['posts', 'published']       // Published posts
   ['posts', 'published', id]   // Specific post
   ```

## Caching

React Query caches data automatically:

```typescript
const { data } = useServerQuery(
  'user',
  getUser,
  {
    staleTime: 5 * 60 * 1000,  // 5 minutes
    gcTime: 10 * 60 * 1000,    // 10 minutes (formerly cacheTime)
    refetchOnWindowFocus: false,
    refetchOnMount: true
  }
)
```

### Default Config

In `@v1/react-query`, defaults are:

```typescript
{
  queries: {
    staleTime: 60 * 1000,        // 1 minute
    retry: 2,
    refetchOnWindowFocus: false
  }
}
```

## Mutations

### Basic Mutation

```typescript
const mutation = useServerMutation(
  createPost,
  {
    onMutate: async (newPost) => {
      // Called before mutation
      console.log('Creating post...')
    },
    onSuccess: (data) => {
      // Called on success
      console.log('Post created:', data)
    },
    onError: (error) => {
      // Called on error
      console.error('Failed:', error)
    },
    onSettled: () => {
      // Called when done (success or error)
      console.log('Mutation complete')
    }
  }
)

// Trigger mutation
mutation.mutate({ title: 'New Post' })
```

### Optimistic Updates

Update UI before server responds:

```typescript
const mutation = useServerMutation(
  updatePost,
  {
    onMutate: async (updatedPost) => {
      // Cancel outgoing refetches
      await queryClient.cancelQueries({ queryKey: ['posts'] })

      // Snapshot previous value
      const previousPosts = queryClient.getQueryData(['posts'])

      // Optimistically update
      queryClient.setQueryData(['posts'], (old) =>
        old.map((post) =>
          post.id === updatedPost.id ? updatedPost : post
        )
      )

      return { previousPosts }
    },
    onError: (err, variables, context) => {
      // Rollback on error
      queryClient.setQueryData(['posts'], context.previousPosts)
    },
    onSettled: () => {
      // Refetch to ensure correct data
      queryClient.invalidateQueries({ queryKey: ['posts'] })
    }
  }
)
```

### Invalidating Queries

Automatically refetch related queries after mutation:

```typescript
const mutation = useServerMutation(
  deletePost,
  {
    onSuccess: () => {
      // Refetch posts list
      queryClient.invalidateQueries({ queryKey: ['posts'] })
    }
  },
  ['posts'] // Or pass as third argument
)
```

## Server Actions Integration

### Creating Server Actions

Server actions must be in separate files with `'use server'`:

```typescript
// actions/posts.ts
'use server'

import { createClient } from '@v1/supabase/server'

export async function getPosts() {
  const supabase = createClient()
  const { data, error } = await supabase
    .from('posts')
    .select('*')

  if (error) throw error
  return data
}

export async function createPost(post: { title: string }) {
  const supabase = createClient()
  const { data, error } = await supabase
    .from('posts')
    .insert(post)
    .select()
    .single()

  if (error) throw error
  return data
}
```

### Using with React Query

```typescript
// components/posts.tsx
'use client'

import { useServerQuery, useServerMutation } from '@v1/react-query'
import { getPosts, createPost } from '@/actions/posts'

export function Posts() {
  const { data } = useServerQuery('posts', getPosts)
  const mutation = useServerMutation(createPost, {}, ['posts'])

  return (
    <div>
      {data?.map((post) => (
        <div key={post.id}>{post.title}</div>
      ))}
      <button onClick={() => mutation.mutate({ title: 'New' })}>
        Add Post
      </button>
    </div>
  )
}
```

## Error Handling

### Query Errors

```typescript
const { data, error, isError } = useServerQuery('user', getUser)

if (isError) {
  return <div>Error: {error.message}</div>
}
```

### Mutation Errors

```typescript
const mutation = useServerMutation(
  createPost,
  {
    onError: (error) => {
      toast.error(error.message)
    }
  }
)

if (mutation.isError) {
  return <div>Error: {mutation.error.message}</div>
}
```

### Global Error Boundary

```typescript
import { QueryErrorResetBoundary } from '@tanstack/react-query'
import { ErrorBoundary } from 'react-error-boundary'

function App() {
  return (
    <QueryErrorResetBoundary>
      {({ reset }) => (
        <ErrorBoundary
          onReset={reset}
          fallbackRender={({ error, resetErrorBoundary }) => (
            <div>
              Error: {error.message}
              <button onClick={resetErrorBoundary}>Try again</button>
            </div>
          )}
        >
          <MyApp />
        </ErrorBoundary>
      )}
    </QueryErrorResetBoundary>
  )
}
```

## DevTools

React Query DevTools are included in development:

```typescript
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

export function Providers({ children }) {
  return (
    <ReactQueryProvider>
      {children}
      {process.env.NODE_ENV === 'development' && (
        <ReactQueryDevtools initialIsOpen={false} />
      )}
    </ReactQueryProvider>
  )
}
```

Access with floating icon in bottom-left corner.

## Best Practices

### 1. Use Query Keys Consistently

```typescript
// Good - consistent structure
const QUERY_KEYS = {
  posts: ['posts'],
  post: (id: string) => ['posts', id],
  userPosts: (userId: string) => ['posts', 'user', userId]
}

useServerQuery(QUERY_KEYS.posts, getPosts)
useServerQuery(QUERY_KEYS.post('123'), () => getPost('123'))
```

### 2. Separate Server Actions

```typescript
// Good - separate file
// actions/posts.ts
'use server'
export async function getPosts() { }

// Bad - inline
const getPosts = async () => { } // Won't work with server actions
```

### 3. Handle Loading States

```typescript
const { data, isLoading, isError } = useServerQuery('posts', getPosts)

if (isLoading) return <Skeleton />
if (isError) return <Error />
return <PostList data={data} />
```

### 4. Use Mutations for Side Effects

```typescript
// Good
const mutation = useServerMutation(createPost)
mutation.mutate({ title: 'New Post' })

// Bad - using query for mutations
useServerQuery('create-post', createPost) // Don't do this
```

### 5. Invalidate Related Queries

```typescript
const mutation = useServerMutation(
  updatePost,
  {},
  ['posts', ['posts', postId]] // Invalidate list and detail
)
```

## Common Patterns

### Dependent Queries

```typescript
const { data: user } = useServerQuery('user', getUser)

const { data: posts } = useServerQuery(
  ['posts', user?.id],
  () => getUserPosts(user!.id),
  {
    enabled: !!user?.id // Only run if user exists
  }
)
```

### Parallel Queries

```typescript
const users = useServerQuery('users', getUsers)
const posts = useServerQuery('posts', getPosts)

// Both run in parallel
```

### Refetch on Interval

```typescript
const { data } = useServerQuery(
  'real-time-data',
  getData,
  {
    refetchInterval: 5000 // Refetch every 5 seconds
  }
)
```

---

**Related Documentation**:
- [Architecture](../current/ARCHITECTURE.md)
- [Supabase Guide](./SUPABASE_GUIDE.md)
- [Getting Started](./GETTING_STARTED.md)
- [TanStack Query Docs](https://tanstack.com/query/latest)
