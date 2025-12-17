import { expect, test } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:4400";
const APP_URL = process.env.APP_URL || "http://localhost:4402";

test.describe("App + API Integration", () => {
  test.describe("API Health from App context", () => {
    test("API is accessible and healthy", async ({ request }) => {
      const response = await request.get(`${API_URL}/health`);
      expect(response.ok()).toBeTruthy();

      const data = await response.json();
      expect(data.status).toBe("ok");
    });
  });

  test.describe("Product Search Integration", () => {
    test("App can search products via API", async ({ request }) => {
      // Simulate app calling API to search products
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
      const searchData = await searchResponse.json();

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
          data: { weight: 5.5 },
        }
      );
      expect(shippingResponse.ok()).toBeTruthy();
      const shippingData = await shippingResponse.json();
      expect(shippingData.cost).toBe(16.5); // 5.5 * 3.0
    });
  });

  test.describe("User Management Integration", () => {
    test("Create and retrieve user", async ({ request }) => {
      // Create user with specific ID
      const userId = `test-user-${Date.now()}`;
      const response = await request.post(`${API_URL}/api/users`, {
        data: { user_id: userId },
      });

      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.id).toBe(userId);
      expect(data.email).toBeDefined();
    });

    test("Generate new user when no ID provided", async ({ request }) => {
      const response = await request.post(`${API_URL}/api/users`, {
        data: {},
      });

      expect(response.ok()).toBeTruthy();
      const data = await response.json();
      expect(data.id).toBeDefined();
      // UUID format check
      expect(data.id).toMatch(
        /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i
      );
    });
  });

  test.describe("Error Handling Integration", () => {
    test("API returns proper error for invalid shipping weight", async ({
      request,
    }) => {
      const response = await request.post(
        `${API_URL}/api/shipping/calculate`,
        {
          data: { weight: -10 },
        }
      );

      expect(response.status()).toBe(400);
    });

    test("API returns validation error for malformed request", async ({
      request,
    }) => {
      const response = await request.post(`${API_URL}/api/orders`, {
        data: {
          // Missing required 'product' field
          quantity: 5,
        },
      });

      expect(response.status()).toBe(422);
    });
  });
});

test.describe("App UI Tests", () => {
  test("App login page loads", async ({ page }) => {
    await page.goto(`${APP_URL}/en/login`);
    await expect(page).toHaveURL(/.*login/);
  });

  test("App redirects unauthenticated users to login", async ({ page }) => {
    await page.goto(`${APP_URL}/en`);
    // Should redirect to login for unauthenticated users
    await expect(page).toHaveURL(/.*login/);
  });
});
