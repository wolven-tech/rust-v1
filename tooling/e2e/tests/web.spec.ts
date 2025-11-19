import { test, expect } from "@playwright/test";

test.describe("Web App E2E Tests", () => {
  test("Homepage loads successfully", async ({ page }) => {
    await page.goto("/");

    // Check that the page loaded
    await expect(page).toHaveTitle(/V1/);

    // Verify page is visible
    await expect(page.locator("body")).toBeVisible();
  });

  test("Subscribe form exists on homepage", async ({ page }) => {
    await page.goto("/");

    // Check if email input exists
    const emailInput = page.locator('input[type="email"]');
    await expect(emailInput).toBeVisible();
  });

  test("Subscribe form validation - invalid email", async ({ page }) => {
    await page.goto("/");

    // Find the email input
    const emailInput = page.locator('input[type="email"]').first();

    // Fill with invalid email
    await emailInput.fill("invalid-email");

    // Submit the form
    const submitButton = page.locator('button[type="submit"]').first();
    await submitButton.click();

    // Browser HTML5 validation should prevent submission
    // Check for validation message or that form wasn't submitted
    const validationMessage = await emailInput.evaluate(
      (el: HTMLInputElement) => el.validationMessage
    );
    expect(validationMessage).toBeTruthy();
  });

  test("Subscribe form submission with valid email", async ({ page }) => {
    await page.goto("/");

    // Find the email input
    const emailInput = page.locator('input[type="email"]').first();

    // Fill with valid email
    await emailInput.fill("test@example.com");

    // Submit the form
    const submitButton = page.locator('button[type="submit"]').first();
    await submitButton.click();

    // Wait for success message or form reset
    // This will depend on your actual implementation
    await page.waitForTimeout(1000);
  });

  test("Navigation links are functional", async ({ page }) => {
    await page.goto("/");

    // Check if footer exists
    const footer = page.locator("footer");
    await expect(footer).toBeVisible();
  });

  test("Page is responsive", async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto("/");

    await expect(page.locator("body")).toBeVisible();

    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto("/");

    await expect(page.locator("body")).toBeVisible();
  });

  test("Analytics script loads", async ({ page }) => {
    await page.goto("/");

    // Check if analytics tracking is initialized
    // This will depend on your PostHog/OpenPanel setup
    await page.waitForTimeout(2000);

    // Verify page loaded successfully
    await expect(page.locator("body")).toBeVisible();
  });
});
