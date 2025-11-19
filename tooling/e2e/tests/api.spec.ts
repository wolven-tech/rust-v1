import { test, expect } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:3002";

test.describe("API E2E Tests", () => {
  test("GET / returns welcome message", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);
    const data = await response.json();

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(data).toEqual({ message: "V1 API is running" });
  });

  test("GET /health returns status", async ({ request }) => {
    const response = await request.get(`${API_URL}/health`);
    const data = await response.json();

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(data.status).toBe("ok");
    expect(data.timestamp).toBeDefined();
  });

  test("GET /swagger returns Swagger documentation", async ({ request }) => {
    const response = await request.get(`${API_URL}/swagger`);

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("text/html");
  });

  test("POST /api/subscribe with valid data", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/subscribe`, {
      data: {
        email: "test@example.com",
        userGroup: "general",
      },
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.success).toBeDefined();
  });

  test("POST /api/subscribe with invalid email", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/subscribe`, {
      data: {
        email: "invalid-email",
        userGroup: "general",
      },
    });

    expect(response.status()).toBe(422);
  });

  test("POST /api/subscribe without email field", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/subscribe`, {
      data: {
        userGroup: "general",
      },
    });

    expect(response.status()).toBe(422);
  });

  test("POST /api/subscribe without userGroup field", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/subscribe`, {
      data: {
        email: "test@example.com",
      },
    });

    expect(response.status()).toBe(422);
  });

  test("CORS headers are present", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);

    expect(response.headers()["access-control-allow-origin"]).toBeDefined();
  });
});
