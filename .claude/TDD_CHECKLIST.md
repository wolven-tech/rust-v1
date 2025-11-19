# TDD Checklist for Claude Code

## MANDATORY: Read Before ANY Feature Work

This checklist MUST be followed for every feature, refactoring, or bug fix. No exceptions.

---

## Definition of Ready (DoR)

Before starting ANY task, verify:

- [ ] Task has clear acceptance criteria
- [ ] Task is E2E testable
- [ ] Dependencies are identified
- [ ] Environment setup is documented
- [ ] You understand the ENTIRE scope of changes needed

**If ANY checkbox is unchecked, STOP and clarify with the user.**

---

## TDD Workflow (RED-GREEN-REFACTOR)

### Phase 1: RED - Write Failing Tests First

**BEFORE writing ANY implementation code:**

1. [ ] **Identify test types needed:**
   - [ ] Unit tests (business logic)
   - [ ] Integration tests (API endpoints, database)
   - [ ] E2E tests (user flows)
   - [ ] Component tests (React components)

2. [ ] **Write tests that FAIL:**
   ```typescript
   // Example: Write test FIRST
   describe('Portfolio route', () => {
     it('redirects to /portfolio when user is logged in', async () => {
       // This will FAIL until we implement it
       const response = await fetch('/');
       expect(response.redirected).toBe(true);
       expect(response.url).toContain('/portfolio');
     });
   });
   ```

3. [ ] **Run tests to verify they FAIL:**
   ```bash
   bun test  # Should FAIL - that's good!
   ```

### Phase 2: GREEN - Make Tests Pass

4. [ ] **Write MINIMAL code to pass tests:**
   - Implement only what's needed to make tests pass
   - No extra features or "nice to haves"

5. [ ] **Run tests again:**
   ```bash
   bun test  # Should PASS now
   ```

6. [ ] **Verify build still works:**
   ```bash
   bun run build
   ```

### Phase 3: REFACTOR - Clean Up

7. [ ] **Improve code quality:**
   - Remove duplication
   - Improve naming
   - Add TypeScript types
   - Optimize performance

8. [ ] **Run tests AGAIN:**
   ```bash
   bun test  # Should still PASS
   ```

---

## Regression Prevention Checklist

### For Route/Navigation Changes:

- [ ] Search for ALL references (case-insensitive):
  ```bash
  # Use Grep tool with -i flag for case-insensitive
  grep -ri "old-route-name" src/
  ```

- [ ] Check these common locations:
  - [ ] Middleware (`src/middleware.ts`)
  - [ ] Navigation/Sidebar components
  - [ ] Link components
  - [ ] Router imports
  - [ ] Test files
  - [ ] Documentation/README
  - [ ] Environment configs

- [ ] **Write tests for:**
  - [ ] Redirect logic in middleware
  - [ ] Navigation menu items
  - [ ] Deep links
  - [ ] Authenticated vs unauthenticated flows

### For Component Changes:

- [ ] **Write component tests:**
  ```typescript
  // Example: Component test
  import { render, screen } from '@testing-library/react';
  import { MyComponent } from './MyComponent';

  describe('MyComponent', () => {
    it('renders without crashing', () => {
      render(<MyComponent />);
      expect(screen.getByText('Expected Text')).toBeInTheDocument();
    });
  });
  ```

- [ ] Test loading states
- [ ] Test error states
- [ ] Test user interactions
- [ ] Test accessibility

### For API/Server Action Changes:

- [ ] **Write integration tests:**
  ```typescript
  // Example: Server action test
  describe('updateExchangeAction', () => {
    it('updates exchange credentials successfully', async () => {
      const result = await updateExchangeAction({
        name: 'Test',
        apiKey: 'key',
        apiSecret: 'secret',
      });
      expect(result.data?.successful).toBe(true);
    });
  });
  ```

---

## Definition of Done (DoD)

Before marking task as complete, verify:

- [ ] ✅ **ALL tests passing** (`bun test`)
- [ ] ✅ **Build successful** (`bun run build`)
- [ ] ✅ **Linters passing** (`bun turbo lint`)
- [ ] ✅ **Type checks passing** (`bun turbo typecheck`)
- [ ] ✅ **No console errors** in browser
- [ ] ✅ **Manual testing** completed
- [ ] ✅ **Documentation updated** (if needed)
- [ ] ✅ **No regressions** (existing features still work)

**If ANY checkbox is unchecked, task is NOT done.**

---

## Test Coverage Requirements

| Type | Minimum Coverage | When Required |
|------|-----------------|---------------|
| Unit Tests | 80% | All business logic |
| Integration Tests | N/A | All API endpoints |
| Component Tests | 80% | All UI components |
| E2E Tests | N/A | Critical user flows |

---

## Example: TDD for Route Rename

```typescript
// ❌ WRONG: Implement first, test later
// 1. Rename directory
// 2. Update imports
// 3. Hope it works

// ✅ CORRECT: Test first
// 1. Write failing test
describe('Portfolio routing', () => {
  it('redirects / to /portfolio when exchange is connected', async () => {
    // Setup: Mock authenticated user with exchange
    const response = await fetch('/', {
      headers: { 'Cookie': 'auth-token=...' }
    });
    expect(response.url).toContain('/portfolio');
  });

  it('redirects / to /portfolio/no-exchange when no exchange', async () => {
    // Setup: Mock authenticated user without exchange
    const response = await fetch('/', {
      headers: { 'Cookie': 'auth-token=...' }
    });
    expect(response.url).toContain('/portfolio/no-exchange');
  });

  it('shows portfolio page for /portfolio route', async () => {
    const response = await fetch('/portfolio');
    expect(response.status).toBe(200);
  });
});

// 2. Run tests - they FAIL (good!)
// 3. Implement minimal code to make tests pass
// 4. Run tests again - they PASS
// 5. Refactor if needed
// 6. Run tests again - still PASS
```

---

## When Tests Can Be Skipped

**NEVER.** Tests cannot be skipped.

If you think you need to skip tests:
1. Stop
2. Re-read this checklist
3. Write the tests

---

## Accountability

**Claude Code commits to:**
- Always following TDD RED-GREEN-REFACTOR cycle
- Writing tests BEFORE implementation
- Running tests and verifying they fail/pass
- Never marking tasks complete without passing DoD checklist
- Asking for clarification if DoR is not met

**User commits to:**
- Providing clear acceptance criteria
- Allowing time for proper TDD workflow
- Reviewing test coverage in PRs
- Holding Claude accountable to this process

---

## Quick Reference Commands

```bash
# Run all tests
bun test

# Run tests in watch mode (TDD)
bun test --watch

# Run specific test file
bun test path/to/file.test.ts

# Run tests with coverage
bun test --coverage

# Type check
bun turbo typecheck

# Lint
bun turbo lint

# Build
bun run build

# Full CI pipeline (run before marking task done)
bun test && bun turbo typecheck && bun turbo lint && bun run build
```

---

**Last Updated:** 2025-10-26
**Version:** 1.0.0
