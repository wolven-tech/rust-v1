import { expect, test } from "@playwright/test";

const APP_URL = process.env.APP_URL || "http://localhost:4402";
const API_URL = process.env.API_URL || "http://localhost:4400";

/**
 * E2E Tests for App Dashboard Features
 *
 * These tests demo the integration between:
 * - @apps/app (Next.js dashboard)
 * - @apps/api (AllFrame-powered API)
 * - @packages/ui (Shared UI components)
 *
 * TDD Approach: Tests written first, then implementation follows.
 */

test.describe("Dashboard - Product Search", () => {
  test("Dashboard page loads with product search section", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Check page has product search section
    await expect(
      page.getByRole("heading", { name: /product search/i }),
    ).toBeVisible();

    // Check for search input (using @rust-v1/ui Input component)
    await expect(page.getByPlaceholder(/search products/i)).toBeVisible();

    // Check for search button (using @rust-v1/ui Button component)
    await expect(page.getByRole("button", { name: /search/i })).toBeVisible();
  });

  test("Can search for products and see results", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Enter search query
    const searchInput = page.getByPlaceholder(/search products/i);
    await searchInput.fill("widget");

    // Click search button
    await page.getByRole("button", { name: /search/i }).click();

    // Wait for results
    await expect(page.getByTestId("search-results")).toBeVisible();

    // Should show results list
    await expect(page.getByTestId("product-item").first()).toBeVisible();
  });

  test("Shows loading state while searching", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    const searchInput = page.getByPlaceholder(/search products/i);
    await searchInput.fill("test");

    await page.getByRole("button", { name: /search/i }).click();

    // Should show loading indicator (using @rust-v1/ui Icons.Loader)
    // Note: This may be brief, so we check it exists in DOM
    const loader = page.getByTestId("search-loading");
    await expect(loader).toBeAttached();
  });
});

test.describe("Dashboard - Shipping Calculator", () => {
  test("Shipping calculator section is visible", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Check for shipping calculator section
    await expect(
      page.getByRole("heading", { name: /shipping calculator/i }),
    ).toBeVisible();

    // Check for weight input
    await expect(page.getByPlaceholder(/weight/i)).toBeVisible();

    // Check for calculate button
    await expect(
      page.getByRole("button", { name: /calculate/i }),
    ).toBeVisible();
  });

  test("Can calculate shipping cost", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Enter weight
    const weightInput = page.getByPlaceholder(/weight/i);
    await weightInput.fill("10");

    // Click calculate
    await page.getByRole("button", { name: /calculate/i }).click();

    // Should show shipping cost result
    await expect(page.getByTestId("shipping-result")).toBeVisible();
    await expect(page.getByTestId("shipping-cost")).toContainText("30"); // 10 * 3.0 = 30
  });

  test("Shows error for invalid weight", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    const weightInput = page.getByPlaceholder(/weight/i);
    await weightInput.fill("-5");

    await page.getByRole("button", { name: /calculate/i }).click();

    // Should show error message
    await expect(page.getByTestId("shipping-error")).toBeVisible();
  });
});

test.describe("Dashboard - User Profile", () => {
  test("User profile section displays user info", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Check for user profile section
    await expect(
      page.getByRole("heading", { name: /user profile/i }),
    ).toBeVisible();

    // Should display user ID
    await expect(page.getByTestId("user-id")).toBeVisible();

    // Should display user name
    await expect(page.getByTestId("user-name")).toBeVisible();

    // Should display user email
    await expect(page.getByTestId("user-email")).toBeVisible();
  });
});

test.describe("Dashboard - UI Components Demo", () => {
  test("Dashboard uses @rust-v1/ui Button variants", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Check for button components with proper styling from @rust-v1/ui
    // Using .first() since there are multiple buttons with the same class
    await expect(page.locator("button.bg-primary").first()).toBeVisible(); // default variant
  });

  test("Dashboard uses @rust-v1/ui Input component", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Check inputs have proper styling from UI package
    const input = page.getByPlaceholder(/search products/i);
    await expect(input).toHaveClass(/border-input/);
  });

  test("Can open order dialog", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Click create order button
    await page.getByRole("button", { name: /create order/i }).click();

    // Dialog should open (using @rust-v1/ui Dialog component)
    const dialog = page.getByRole("dialog");
    await expect(dialog).toBeVisible();

    // Dialog should have form fields
    await expect(dialog.getByPlaceholder(/product name/i)).toBeVisible();
    await expect(dialog.getByPlaceholder(/quantity/i)).toBeVisible();
  });

  test("Can create order via dialog", async ({ page }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Open dialog
    await page.getByRole("button", { name: /create order/i }).click();

    const dialog = page.getByRole("dialog");

    // Fill form
    await dialog.getByPlaceholder(/product name/i).fill("Widget Pro");
    await dialog.getByPlaceholder(/quantity/i).fill("3");

    // Submit
    await dialog.getByRole("button", { name: /submit order/i }).click();

    // Should show success message
    await expect(page.getByTestId("order-success")).toBeVisible();
    await expect(page.getByTestId("order-id")).toBeVisible();
  });
});

test.describe("Dashboard - API Integration", () => {
  test("Dashboard fetches data from API on load", async ({ page, request }) => {
    // First verify API is running
    const healthCheck = await request.get(`${API_URL}/health`);
    expect(healthCheck.ok()).toBeTruthy();

    await page.goto(`${APP_URL}/en/dashboard`);

    // User profile should be loaded from API
    await expect(page.getByTestId("user-id")).not.toBeEmpty();
  });

  test("Search results come from API", async ({ page, request }) => {
    await page.goto(`${APP_URL}/en/dashboard`);

    // Perform search
    await page.getByPlaceholder(/search products/i).fill("test");
    await page.getByRole("button", { name: /search/i }).click();

    // Results should be displayed
    await expect(page.getByTestId("search-results")).toBeVisible();
  });
});
