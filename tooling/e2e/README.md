# E2E Testing with Playwright

End-to-end tests for the V1 application using Playwright Test.

## Features

- 🎭 **Playwright** - Fast, reliable end-to-end testing
- 🌐 **Cross-browser** - Test on Chromium, Firefox, and WebKit
- 📸 **Screenshots & Videos** - Automatic failure artifacts
- 🔄 **Auto-wait** - No flaky tests with smart waiting
- 🎯 **API Testing** - Built-in API testing capabilities
- 🐛 **Debug Mode** - Interactive debugging with Playwright Inspector

## Setup

```bash
# Install dependencies
bun install

# Install Playwright browsers
bunx playwright install
```

## Running Tests

```bash
# Run all E2E tests (headless)
bun test

# Run only API tests
bun run test:api

# Run only web tests
bun run test:web

# Run with browser UI visible
bun run test:headed

# Debug mode with Playwright Inspector
bun run test:debug

# Interactive UI mode
bun run test:ui

# View test report
bun run report
```

## Test Environment

Before running tests, start the required services:

```bash
# Terminal 1: Start API server
cd apps/api
bun run dev

# Terminal 2: Start web server
cd apps/web
bun run dev

# Terminal 3: Run tests
cd tooling/e2e
bun test
```

Default URLs:
- **API**: `http://localhost:3002`
- **Web**: `http://localhost:3001`

You can override these URLs with environment variables:
- `API_URL` - Custom API URL
- `WEB_URL` - Custom web app URL

**Note**: To enable auto-start servers in `playwright.config.ts`, uncomment the `webServer` configuration.

## Writing Tests

### API Tests

API tests use Playwright's built-in request context:

```typescript
import { test, expect } from "@playwright/test";

test("API endpoint works", async ({ request }) => {
  const response = await request.get("http://localhost:3002/api/endpoint");

  expect(response.ok()).toBeTruthy();
  expect(response.status()).toBe(200);

  const data = await response.json();
  expect(data).toBeDefined();
});
```

### Web Tests

Web tests use Playwright's page object:

```typescript
import { test, expect } from "@playwright/test";

test("Homepage loads", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle(/V1/);
  await expect(page.locator("body")).toBeVisible();
});

test("Form submission", async ({ page }) => {
  await page.goto("/");

  await page.fill('input[type="email"]', "test@example.com");
  await page.click('button[type="submit"]');

  await expect(page.locator(".success-message")).toBeVisible();
});
```

## Test Structure

```
tooling/e2e/
├── tests/
│   ├── api.spec.ts        # API endpoint tests
│   └── web.spec.ts        # Web application tests
├── playwright.config.ts   # Playwright configuration
└── package.json
```

## Configuration

Edit `playwright.config.ts` to customize:
- Test directory
- Browsers to test
- Viewport sizes
- Timeout settings
- Screenshot/video options

## Debugging

### Debug a specific test

```bash
bunx playwright test --debug api.spec.ts
```

### Generate test code

```bash
bunx playwright codegen http://localhost:3001
```

### View trace files

```bash
bunx playwright show-trace trace.zip
```

## CI/CD

These tests run in the CI pipeline:

1. Unit tests pass
2. Build succeeds
3. **E2E tests run** ← You are here
4. Deploy

### GitHub Actions Example

```yaml
- name: Install Playwright
  run: bunx playwright install --with-deps

- name: Run E2E tests
  run: |
    cd tooling/e2e
    bun test

- name: Upload test results
  if: always()
  uses: actions/upload-artifact@v3
  with:
    name: playwright-report
    path: tooling/e2e/playwright-report/
```

## Best Practices

1. **Use data-testid attributes** for stable selectors
2. **Avoid hard waits** - use Playwright's auto-waiting
3. **Test user flows**, not implementation details
4. **Keep tests independent** - no test should depend on another
5. **Use Page Object Model** for complex pages
6. **Mock external APIs** when needed

## Troubleshooting

### Tests timing out

Increase timeout in `playwright.config.ts`:

```typescript
use: {
  actionTimeout: 30000,
  navigationTimeout: 30000,
}
```

### Browsers not found

```bash
bunx playwright install
```

### Server not starting

Check that ports 3001 and 3002 are available:

```bash
lsof -i :3001
lsof -i :3002
```

## Resources

- [Playwright Documentation](https://playwright.dev)
- [Best Practices](https://playwright.dev/docs/best-practices)
- [API Testing Guide](https://playwright.dev/docs/api-testing)
