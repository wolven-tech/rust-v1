# React & TypeScript Best Practices Skill

## Purpose
This skill ensures high-quality React and TypeScript development following modern best practices, component purity, and Next.js App Router patterns.

## When to Use
Activate this skill when:
- Writing React components (Server or Client)
- Implementing hooks
- Working with Next.js App Router
- Building UI with Tailwind CSS and Shadcn UI
- Writing TypeScript code in the frontend

---

## 🎯 Core Principles

### 1. Component & Hook Purity (MANDATORY)

React components and hooks **MUST** be pure functions:

#### ✅ Pure Functions Are:
- **Idempotent**: Same inputs → same outputs, every time
- **Side-effect free during render**: No mutations, no API calls, no timers
- **Isolated**: Don't mutate external state during render

#### ❌ Common Purity Violations

**DON'T: Call non-idempotent functions in render**
```typescript
// ❌ WRONG - Different result each call
function Clock() {
  const time = new Date();
  return <span>{time.toLocaleString()}</span>;
}
```

**DO: Move to Effects or event handlers**
```typescript
// ✅ CORRECT - Effect synchronizes with external system
function Clock() {
  const [time, setTime] = useState(() => new Date());

  useEffect(() => {
    const id = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(id);
  }, []);

  return <span>{time.toLocaleString()}</span>;
}
```

**DON'T: Mutate props or non-local state**
```typescript
// ❌ WRONG - Mutates prop
function Post({ item }: { item: PostItem }) {
  item.url = new URL(item.url, base); // Mutation!
  return <Link url={item.url}>{item.title}</Link>;
}

// ❌ WRONG - Mutates state directly
function Counter() {
  const [count, setCount] = useState(0);
  function handleClick() {
    count = count + 1; // Direct mutation!
  }
}
```

**DO: Create new values**
```typescript
// ✅ CORRECT - New value
function Post({ item }: { item: PostItem }) {
  const url = new URL(item.url, base);
  return <Link url={url}>{item.title}</Link>;
}

// ✅ CORRECT - Use setter
function Counter() {
  const [count, setCount] = useState(0);
  function handleClick() {
    setCount(count + 1);
  }
}
```

**Local mutation IS allowed**
```typescript
// ✅ CORRECT - Local variable created fresh each render
function FriendList({ friends }: { friends: Friend[] }) {
  const items = []; // Local, not mutating external state
  for (const friend of friends) {
    items.push(<Friend key={friend.id} friend={friend} />);
  }
  return <section>{items}</section>;
}
```

### 2. Side Effects Belong in Event Handlers or Effects

**NEVER in render:**
```typescript
// ❌ WRONG - Side effect during render
function Component() {
  console.log("Rendering!"); // Runs multiple times unpredictably
  fetch("/api/data"); // Triggers on every render!
  return <div>...</div>;
}
```

**DO: Use event handlers for user actions**
```typescript
// ✅ CORRECT - Side effect in event handler
function Component() {
  const handleClick = async () => {
    console.log("Button clicked");
    await fetch("/api/data");
  };
  return <button onClick={handleClick}>Click</button>;
}
```

**DO: Use Effects for synchronization**
```typescript
// ✅ CORRECT - Effect for external sync
function Component() {
  useEffect(() => {
    console.log("Component mounted");
    const subscription = dataSource.subscribe();
    return () => subscription.unsubscribe();
  }, []);

  return <div>...</div>;
}
```

### 3. Hook Arguments & JSX Must Be Immutable

**DON'T: Mutate hook arguments**
```typescript
// ❌ WRONG - Mutates object passed to hook
function useIconStyle(icon: Icon) {
  icon.className = computeStyle(icon); // Breaks memoization!
  return icon;
}
```

**DO: Create new values**
```typescript
// ✅ CORRECT - New object
function useIconStyle(icon: Icon) {
  return {
    ...icon,
    className: computeStyle(icon),
  };
}
```

**DON'T: Mutate after JSX creation**
```typescript
// ❌ WRONG - Mutation after JSX
const styles = { size: "large" };
const header = <Header styles={styles} />;
styles.size = "small"; // Violates immutability!
```

**DO: Use separate objects**
```typescript
// ✅ CORRECT - Separate objects
const headerStyles = { size: "large" };
const header = <Header styles={headerStyles} />;
const footerStyles = { size: "small" };
const footer = <Footer styles={footerStyles} />;
```

---

## 🏗️ Next.js App Router Patterns

### Server Components by Default

```typescript
// ✅ CORRECT - Server Component (default)
// No 'use client' directive needed
interface PageProps {
  params: { id: string };
}

export default async function ProductPage({ params }: PageProps) {
  // Direct data fetching - no useEffect needed
  const product = await getProduct(params.id);

  return (
    <div>
      <h1>{product.name}</h1>
      <ProductClient data={product} />
    </div>
  );
}
```

### Client Components (Minimal Usage)

```typescript
// ✅ CORRECT - Client component only when needed
'use client';

import { useState } from 'react';

interface ProductClientProps {
  data: Product;
}

export function ProductClient({ data }: ProductClientProps) {
  const [count, setCount] = useState(0);

  return (
    <button onClick={() => setCount(count + 1)}>
      {data.name}: {count}
    </button>
  );
}
```

### Component Structure

```typescript
// ✅ CORRECT - Clean structure with early returns
interface UserCardProps {
  userId: string;
  isLoading?: boolean;
  error?: Error;
}

export function UserCard({ userId, isLoading, error }: UserCardProps) {
  // Early returns for edge cases
  if (error) return <ErrorState error={error} />;
  if (isLoading) return <LoadingState />;
  if (!userId) return null;

  // Main render
  return (
    <div className="rounded-lg border p-4">
      <UserAvatar id={userId} />
      <UserInfo id={userId} />
    </div>
  );
}
```

---

## 📝 TypeScript Best Practices

### Prefer Interfaces Over Types

```typescript
// ✅ CORRECT - Interface for object shapes
interface UserProps {
  name: string;
  email: string;
  isActive: boolean;
}

// ✅ CORRECT - Type for unions/primitives
type Status = 'pending' | 'active' | 'inactive';
type ID = string | number;

// ✅ CORRECT - Interface can extend
interface AdminProps extends UserProps {
  role: 'admin' | 'superadmin';
}
```

### Functional Components Only

```typescript
// ✅ CORRECT - Functional component with interface
interface ButtonProps {
  variant: 'primary' | 'secondary';
  onClick: () => void;
  children: React.ReactNode;
}

export function Button({ variant, onClick, children }: ButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "rounded px-4 py-2",
        variant === 'primary' && "bg-blue-500 text-white",
        variant === 'secondary' && "bg-gray-200 text-gray-900"
      )}
    >
      {children}
    </button>
  );
}

// ❌ WRONG - Class components are deprecated
class Button extends React.Component<ButtonProps> { ... }
```

### Proper Error Handling

```typescript
// ✅ CORRECT - Discriminated unions for results
type Result<T> =
  | { success: true; data: T }
  | { success: false; error: string };

async function fetchUser(id: string): Promise<Result<User>> {
  try {
    const data = await api.getUser(id);
    return { success: true, data };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : 'Unknown error',
    };
  }
}

// Usage with type narrowing
const result = await fetchUser('123');
if (result.success) {
  console.log(result.data.name); // TypeScript knows data exists
} else {
  console.error(result.error); // TypeScript knows error exists
}
```

---

## 🎨 Styling with Tailwind CSS

### Use Utility Classes

```typescript
// ✅ CORRECT - Tailwind utility classes
export function Card({ children }: { children: React.ReactNode }) {
  return (
    <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm">
      {children}
    </div>
  );
}
```

### Conditional Classes with cn()

```typescript
import { cn } from '@v1/ui/cn';

// ✅ CORRECT - cn() for conditional classes
interface BadgeProps {
  variant: 'default' | 'success' | 'error';
  children: React.ReactNode;
}

export function Badge({ variant, children }: BadgeProps) {
  return (
    <span
      className={cn(
        "inline-flex items-center rounded-full px-2 py-1 text-xs font-medium",
        variant === 'default' && "bg-gray-100 text-gray-800",
        variant === 'success' && "bg-green-100 text-green-800",
        variant === 'error' && "bg-red-100 text-red-800"
      )}
    >
      {children}
    </span>
  );
}
```

### Mobile-First Responsive Design

```typescript
// ✅ CORRECT - Mobile-first with breakpoints
export function Hero() {
  return (
    <section className="
      px-4 py-8           /* Mobile: small padding */
      sm:px-6 sm:py-12   /* Small: medium padding */
      md:px-8 md:py-16   /* Medium: large padding */
      lg:px-12 lg:py-24  /* Large: extra padding */
    ">
      <h1 className="
        text-2xl          /* Mobile: 24px */
        sm:text-3xl       /* Small: 30px */
        md:text-4xl       /* Medium: 36px */
        lg:text-5xl       /* Large: 48px */
        font-bold
      ">
        Welcome
      </h1>
    </section>
  );
}
```

---

## 🔄 State Management

### URL Params with nuqs

```typescript
'use client';

import { useQueryState } from 'nuqs';

// ✅ CORRECT - URL state for shareable state
export function SearchComponent() {
  const [query, setQuery] = useQueryState('q');
  const [filter, setFilter] = useQueryState('filter');

  return (
    <div>
      <input
        value={query || ''}
        onChange={(e) => setQuery(e.target.value)}
        placeholder="Search..."
      />
      <select value={filter || 'all'} onChange={(e) => setFilter(e.target.value)}>
        <option value="all">All</option>
        <option value="active">Active</option>
      </select>
    </div>
  );
}
```

### React Query for Server State

```typescript
'use client';

import { useQuery } from '@tanstack/react-query';

// ✅ CORRECT - React Query for server data
export function UserProfile({ userId }: { userId: string }) {
  const { data, isLoading, error } = useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetchUser(userId),
  });

  if (isLoading) return <Skeleton />;
  if (error) return <ErrorState error={error} />;
  if (!data) return null;

  return <UserCard user={data} />;
}
```

### Avoid useState/useEffect Where Possible

```typescript
// ❌ WRONG - Derived state in useState
function Component({ items }: { items: Item[] }) {
  const [count, setCount] = useState(items.length);

  useEffect(() => {
    setCount(items.length);
  }, [items]);

  return <div>Count: {count}</div>;
}

// ✅ CORRECT - Calculate during render
function Component({ items }: { items: Item[] }) {
  const count = items.length; // Derived, no state needed
  return <div>Count: {count}</div>;
}
```

---

## 🧪 Server Actions

```typescript
'use server';

import { z } from 'zod';
import { actionClient } from '@/lib/safe-action';

// ✅ CORRECT - Server action with validation
const schema = z.object({
  name: z.string().min(1),
  email: z.string().email(),
});

export const createUser = actionClient
  .schema(schema)
  .action(async ({ parsedInput: { name, email } }) => {
    const user = await db.user.create({
      data: { name, email },
    });

    revalidatePath('/users');
    return { success: true, user };
  });
```

```typescript
'use client';

// ✅ CORRECT - Using server action in client component
import { createUser } from './actions';

export function CreateUserForm() {
  const [isPending, startTransition] = useTransition();

  const handleSubmit = async (formData: FormData) => {
    startTransition(async () => {
      const result = await createUser({
        name: formData.get('name') as string,
        email: formData.get('email') as string,
      });

      if (result.success) {
        toast.success('User created!');
      }
    });
  };

  return (
    <form action={handleSubmit}>
      <input name="name" required />
      <input name="email" type="email" required />
      <button disabled={isPending}>
        {isPending ? 'Creating...' : 'Create User'}
      </button>
    </form>
  );
}
```

---

## ♿ Accessibility

```typescript
// ✅ CORRECT - Accessible component using Radix UI
import * as Dialog from '@radix-ui/react-dialog';

export function AccessibleDialog() {
  return (
    <Dialog.Root>
      <Dialog.Trigger asChild>
        <button className="btn-primary">
          Open Dialog
        </button>
      </Dialog.Trigger>

      <Dialog.Portal>
        <Dialog.Overlay className="fixed inset-0 bg-black/50" />
        <Dialog.Content className="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
          <Dialog.Title className="text-lg font-semibold">
            Dialog Title
          </Dialog.Title>
          <Dialog.Description className="text-sm text-gray-600">
            Description for screen readers
          </Dialog.Description>

          {/* Content */}

          <Dialog.Close asChild>
            <button aria-label="Close dialog">×</button>
          </Dialog.Close>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
```

---

## 🚀 Performance

### Code Splitting

```typescript
// ✅ CORRECT - Dynamic imports for code splitting
import dynamic from 'next/dynamic';

const HeavyComponent = dynamic(() => import('./HeavyComponent'), {
  loading: () => <Skeleton />,
  ssr: false, // Client-side only if needed
});

export function Page() {
  return (
    <div>
      <LightComponent />
      <HeavyComponent />
    </div>
  );
}
```

### Image Optimization

```typescript
import Image from 'next/image';

// ✅ CORRECT - Optimized images
export function ProductImage({ src, alt }: { src: string; alt: string }) {
  return (
    <Image
      src={src}
      alt={alt}
      width={800}
      height={600}
      placeholder="blur"
      blurDataURL="data:image/..."
      className="rounded-lg"
    />
  );
}
```

### React.memo() (Use Sparingly)

```typescript
// ✅ CORRECT - memo only when measured benefit
import { memo } from 'react';

interface ExpensiveListItemProps {
  item: ComplexItem;
  onSelect: (id: string) => void;
}

export const ExpensiveListItem = memo(function ExpensiveListItem({
  item,
  onSelect,
}: ExpensiveListItemProps) {
  // Expensive rendering logic
  return <div onClick={() => onSelect(item.id)}>{item.name}</div>;
});

// ❌ WRONG - Don't memo everything blindly
export const SimpleComponent = memo(function SimpleComponent() {
  return <div>Hello</div>; // Too simple to benefit
});
```

---

## 📦 Component Development Workflow

### 1. Design System First (packages/ui)

```typescript
// packages/ui/src/components/button.tsx
import { cn } from '../lib/cn';
import { cva, type VariantProps } from 'class-variance-authority';

const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md font-medium transition-colors",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/90",
        destructive: "bg-destructive text-destructive-foreground hover:bg-destructive/90",
        outline: "border border-input bg-background hover:bg-accent",
      },
      size: {
        default: "h-10 px-4 py-2",
        sm: "h-9 px-3",
        lg: "h-11 px-8",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

export function Button({ className, variant, size, ...props }: ButtonProps) {
  return (
    <button
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
}
```

### 2. Storybook Stories

```typescript
// packages/ui/src/components/button.stories.tsx
import type { Meta, StoryObj } from '@storybook/react';
import { Button } from './button';

const meta: Meta<typeof Button> = {
  title: 'Components/Button',
  component: Button,
  tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof Button>;

export const Default: Story = {
  args: {
    children: 'Button',
  },
};

export const Destructive: Story = {
  args: {
    variant: 'destructive',
    children: 'Delete',
  },
};

export const Outline: Story = {
  args: {
    variant: 'outline',
    children: 'Cancel',
  },
};
```

### 3. App Integration

```typescript
// apps/app/src/components/user-actions.tsx
import { Button } from '@v1/ui/button';

export function UserActions({ userId }: { userId: string }) {
  return (
    <div className="flex gap-2">
      <Button variant="default" onClick={() => handleEdit(userId)}>
        Edit
      </Button>
      <Button variant="destructive" onClick={() => handleDelete(userId)}>
        Delete
      </Button>
    </div>
  );
}
```

---

## 🚫 Anti-Patterns to Avoid

### ❌ DON'T:
- Mutate props or state directly
- Call APIs during render
- Use `any` type in TypeScript
- Create class components
- Use `useEffect` for derived state
- Forget error boundaries
- Skip loading states
- Ignore accessibility
- Use inline styles (use Tailwind)
- Put business logic in components

### ✅ DO:
- Keep components pure
- Use proper TypeScript types
- Handle loading and error states
- Follow mobile-first design
- Use semantic HTML
- Test components in Storybook
- Optimize images
- Code split heavy components
- Use Server Components by default
- Follow TDD practices

---

## Quick Reference Checklist

Before committing React/TypeScript code:

- [ ] Components are pure (no side effects in render)
- [ ] TypeScript types are explicit (no `any`)
- [ ] Interfaces used for object shapes
- [ ] Loading and error states handled
- [ ] Accessibility attributes added
- [ ] Tailwind classes used (no inline styles)
- [ ] Mobile-first responsive design
- [ ] Images optimized with next/image
- [ ] Server Components used where possible
- [ ] Storybook stories created (for UI components)
- [ ] Tests written and passing
- [ ] No console errors in browser

---

**Remember**: Pure components → Predictable behavior → Maintainable codebase
