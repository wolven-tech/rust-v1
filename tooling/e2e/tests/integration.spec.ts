import { expect, test } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:4400";
const APP_URL = process.env.APP_URL || "http://localhost:4402";
const WEB_URL = process.env.WEB_URL || "http://localhost:4401";

test.describe("Full Stack Integration", () => {
  test.describe("API Health from App context", () => {
    test("API is accessible and healthy", async ({ request }) => {
      const response = await request.get(`${API_URL}/health`);
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data.status).toBe("ok");
      expect(data.timestamp).toBeDefined();
      expect(data.version).toBeDefined();
    });

    test("API documentation is accessible", async ({ request }) => {
      const response = await request.get(`${API_URL}/docs`);
      expect(response.ok()).toBeTruthy();
      expect(response.headers()["content-type"]).toContain("text/html");
    });
  });

  test.describe("Product Search Integration", () => {
    test("App can search products via API", async ({ request }) => {
      const response = await request.post(`${API_URL}/api/products/search`, {
        data: { query: "widget" },
      });

      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.results).toBeDefined();
      expect(Array.isArray(data.results)).toBeTruthy();
    });
  });

  test.describe("Order Creation Flow", () => {
    test("Complete order flow: search -> create order -> calculate shipping", async ({
      request,
    }) => {
      // Step 1: Search for products
      const searchResponse = await request.post(
        `${API_URL}/api/products/search`,
        {
          data: { query: "premium" },
        }
      );
      expect(searchResponse.ok()).toBeTruthy();

      // Step 2: Create an order
      const orderResponse = await request.post(`${API_URL}/api/orders`, {
        data: {
          product: "Premium Widget",
          quantity: 3,
        },
      });
      expect(orderResponse.ok()).toBeTruthy();
      const orderData = await orderResponse.json();
      expect(orderData.order_id).toBeDefined();
      expect(orderData.status).toBe("created");

      // Step 3: Calculate shipping for the order
      const shippingResponse = await request.post(
        `${API_URL}/api/shipping/calculate`,
        {
          data: { weight: 10.0 },
        }
      );
      expect(shippingResponse.ok()).toBeTruthy();
      const shippingData = await shippingResponse.json();
      expect(shippingData.weight).toBe(10.0);
      expect(shippingData.cost).toBe(30.0);
    });
  });

  test.describe("User Management Integration", () => {
    test("Get user returns user data", async ({ request }) => {
      const response = await request.post(`${API_URL}/api/users`, {
        data: {},
      });

      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.id).toBeDefined();
      expect(data.name).toBeDefined();
      expect(data.email).toBeDefined();
    });
  });

  test.describe("Subscription Integration", () => {
    test("Subscribe endpoint returns result", async ({ request }) => {
      const response = await request.post(`${API_URL}/api/subscribe`, {
        data: {
          email: "test@example.com",
          userGroup: "general",
        },
      });

      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.success).toBeDefined();
    });
  });
});

test.describe("App UI Tests", () => {
  test("App login page loads", async ({ page }) => {
    await page.goto(`${APP_URL}/en/login`);

    // Check page loaded
    await expect(page.locator("body")).toBeVisible();

    // Check for logo
    const logo = page.locator('img[alt="logo"]');
    await expect(logo).toBeVisible();

    // Check for Google Sign-in button
    const signInButton = page.getByRole("button", { name: /sign in/i });
    await expect(signInButton).toBeVisible();
  });

  test("App redirects unauthenticated users to login", async ({ page }) => {
    await page.goto(`${APP_URL}/en`);

    // Should redirect to login for unauthenticated users
    await expect(page).toHaveURL(/.*login/);
  });
});

test.describe("Web App Integration", () => {
  test("Web homepage loads", async ({ page }) => {
    await page.goto(WEB_URL);

    await expect(page.locator("body")).toBeVisible();
    await expect(page.locator("h1")).toBeVisible();
  });

  test("Web app can trigger subscribe form", async ({ page }) => {
    await page.goto(WEB_URL);

    // Click Get updates
    const getUpdatesButton = page.locator('header span:has-text("Get updates")');
    await getUpdatesButton.click();

    // Check dialog appears with email input
    const dialog = page.locator('[role="dialog"]');
    await expect(dialog).toBeVisible();
    await expect(dialog.locator('input[type="email"]')).toBeVisible();
  });
});

test.describe("Cross-Service Communication", () => {
  test("All services are responsive", async ({ request, page }) => {
    // Check API
    const apiResponse = await request.get(`${API_URL}/health`);
    expect(apiResponse.ok()).toBeTruthy();

    // Check Web
    await page.goto(WEB_URL);
    await expect(page.locator("body")).toBeVisible();

    // Check App
    await page.goto(`${APP_URL}/en/login`);
    await expect(page.locator("body")).toBeVisible();
  });

  test("API CORS allows cross-origin requests", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);

    expect(response.ok()).toBeTruthy();
    expect(response.headers()["access-control-allow-origin"]).toBe("*");
  });
});
