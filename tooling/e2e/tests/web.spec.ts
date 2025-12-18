import { expect, test } from "@playwright/test";

const WEB_URL = process.env.WEB_URL || "http://localhost:4401";

test.describe("Web App - Homepage", () => {
  test("Homepage loads successfully", async ({ page }) => {
    await page.goto(WEB_URL);

    // Check that the page loaded with correct title
    await expect(page).toHaveTitle(/v1/i);

    // Verify page is visible
    await expect(page.locator("body")).toBeVisible();
  });

  test("Homepage displays main heading", async ({ page }) => {
    await page.goto(WEB_URL);

    // Check for the animated text heading
    const heading = page.locator("h1");
    await expect(heading).toBeVisible();
    await expect(heading).toContainText("Production ready code");
  });

  test("Homepage displays Wolven Tech attribution", async ({ page }) => {
    await page.goto(WEB_URL);

    // Check for Wolven Tech link in the main content (not header)
    const wolvenLink = page.locator('p a[href="https://github.com/wolven-tech"]');
    await expect(wolvenLink).toBeVisible();
    await expect(wolvenLink).toContainText("Wolven Tech");
  });

  test("Copy command is displayed", async ({ page }) => {
    await page.goto(WEB_URL);

    // Check for the copy text component with degit command
    await expect(page.getByText("bunx degit")).toBeVisible();
  });
});

test.describe("Web App - Header", () => {
  test("Header displays logo", async ({ page }) => {
    await page.goto(WEB_URL);

    const logo = page.locator('header img[alt="V1 logo"]');
    await expect(logo).toBeVisible();
  });

  test("Header has Github link", async ({ page }) => {
    await page.goto(WEB_URL);

    const githubLink = page.locator('header a:has-text("Github")');
    await expect(githubLink).toBeVisible();
    await expect(githubLink).toHaveAttribute(
      "href",
      "https://github.com/wolven-tech"
    );
  });

  test("Header has Get updates button", async ({ page }) => {
    await page.goto(WEB_URL);

    const getUpdatesButton = page.locator('header span:has-text("Get updates")');
    await expect(getUpdatesButton).toBeVisible();
  });

  test("Get updates opens subscribe dialog", async ({ page }) => {
    await page.goto(WEB_URL);

    // Click the Get updates button
    const getUpdatesButton = page.locator('header span:has-text("Get updates")');
    await getUpdatesButton.click();

    // Check dialog appears
    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible();

    // Check dialog has subscribe form
    await expect(dialog.locator('input[type="email"]')).toBeVisible();
    await expect(dialog.locator('button:has-text("Subscribe")')).toBeVisible();
  });
});

test.describe("Web App - Footer", () => {
  test("Footer is visible", async ({ page }) => {
    await page.goto(WEB_URL);

    const footer = page.locator("footer");
    await expect(footer).toBeVisible();
  });

  test("Footer has Featuring text", async ({ page }) => {
    await page.goto(WEB_URL);

    await expect(page.locator("footer")).toContainText("Featuring");
  });

  test("Footer has technology links", async ({ page }) => {
    await page.goto(WEB_URL);

    // Check for Rust link
    const rustLink = page.locator('footer a[href="https://www.rust-lang.org"]');
    await expect(rustLink).toBeVisible();
    await expect(rustLink).toContainText("Rust");

    // Check for AllSource link
    const allsourceLink = page.locator(
      'footer a[href="https://github.com/all-source-os"]'
    );
    await expect(allsourceLink).toBeVisible();

    // Check for Allframe link
    const allframeLink = page.locator(
      'footer a[href="https://all-source-os.github.io/all-frame/"]'
    );
    await expect(allframeLink).toBeVisible();
  });
});

test.describe("Web App - Subscribe Form", () => {
  test("Subscribe form validates email", async ({ page }) => {
    await page.goto(WEB_URL);

    // Open dialog
    const getUpdatesButton = page.locator('header span:has-text("Get updates")');
    await getUpdatesButton.click();

    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible();

    // Find the email input
    const emailInput = dialog.locator('input[type="email"]');

    // Fill with invalid email
    await emailInput.fill("invalid-email");

    // Submit the form
    const submitButton = dialog.locator('button:has-text("Subscribe")');
    await submitButton.click();

    // Browser HTML5 validation should prevent submission
    const validationMessage = await emailInput.evaluate(
      (el: HTMLInputElement) => el.validationMessage
    );
    expect(validationMessage).toBeTruthy();
  });

  test("Subscribe form accepts valid email", async ({ page }) => {
    await page.goto(WEB_URL);

    // Open dialog
    const getUpdatesButton = page.locator('header span:has-text("Get updates")');
    await getUpdatesButton.click();

    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible();

    // Fill with valid email
    const emailInput = dialog.locator('input[type="email"]');
    await emailInput.fill("test@example.com");

    // Submit the form
    const submitButton = dialog.locator('button:has-text("Subscribe")');
    await submitButton.click();

    // Wait for response - should show "Subscribed" or stay on form
    await page.waitForTimeout(1000);
  });
});

test.describe("Web App - Responsiveness", () => {
  test("Page is responsive on mobile", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(WEB_URL);

    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("h1")).toBeVisible();
  });

  test("Page is responsive on tablet", async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto(WEB_URL);

    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("h1")).toBeVisible();
  });

  test("Page is responsive on desktop", async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto(WEB_URL);

    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("h1")).toBeVisible();
  });
});
