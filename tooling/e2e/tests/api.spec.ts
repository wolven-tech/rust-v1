import { expect, test } from "@playwright/test";

const API_URL = process.env.API_URL || "http://localhost:4400";

test.describe("API Health & Root", () => {
  test("GET / returns welcome message and version", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);
    const data = await response.json();

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(data.message).toBe("V1 API is running");
    expect(data.version).toBeDefined();
  });

  test("GET /health returns status", async ({ request }) => {
    const response = await request.get(`${API_URL}/health`);
    const data = await response.json();

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(data.status).toBe("ok");
    expect(data.timestamp).toBeDefined();
    expect(data.version).toBeDefined();
  });

  test("GET /api/docs returns API documentation", async ({ request }) => {
    const response = await request.get(`${API_URL}/api/docs`);

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("text/html");
  });

  test("CORS headers are present", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);

    expect(response.headers()["access-control-allow-origin"]).toBeDefined();
  });
});

test.describe("Products API", () => {
  test("POST /api/products/search returns products", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/products/search`, {
      data: {
        query: "test",
      },
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.query).toBe("test");
    expect(data.results).toBeDefined();
    expect(Array.isArray(data.results)).toBeTruthy();
  });

  test("POST /api/products/search with empty query", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/products/search`, {
      data: {
        query: "",
      },
    });

    expect(response.ok()).toBeTruthy();
    const data = await response.json();
    expect(data.query).toBe("");
  });
});

test.describe("Orders API", () => {
  test("POST /api/orders creates an order", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/orders`, {
      data: {
        product: "Widget",
        quantity: 2,
      },
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.order_id).toBeDefined();
    expect(data.product).toBe("Widget");
    expect(data.status).toBe("created");
  });

  test("POST /api/orders with missing product field returns error", async ({
    request,
  }) => {
    const response = await request.post(`${API_URL}/api/orders`, {
      data: {
        quantity: 2,
      },
    });

    expect(response.status()).toBe(422);
  });
});

test.describe("Shipping API", () => {
  test("POST /api/shipping/calculate returns shipping cost", async ({
    request,
  }) => {
    const response = await request.post(`${API_URL}/api/shipping/calculate`, {
      data: {
        weight: 10.0,
      },
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.weight).toBe(10.0);
    expect(data.cost).toBe(30.0); // $3 per unit weight
  });

  test("POST /api/shipping/calculate with zero weight returns error", async ({
    request,
  }) => {
    const response = await request.post(`${API_URL}/api/shipping/calculate`, {
      data: {
        weight: 0,
      },
    });

    expect(response.status()).toBe(400);
  });

  test("POST /api/shipping/calculate with negative weight returns error", async ({
    request,
  }) => {
    const response = await request.post(`${API_URL}/api/shipping/calculate`, {
      data: {
        weight: -5.0,
      },
    });

    expect(response.status()).toBe(400);
  });
});

test.describe("Users API", () => {
  test("POST /api/users returns user data", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/users`, {
      data: {
        user_id: "test-user-123",
      },
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.id).toBe("test-user-123");
    expect(data.name).toBeDefined();
    expect(data.email).toBeDefined();
  });

  test("POST /api/users without user_id generates new id", async ({
    request,
  }) => {
    const response = await request.post(`${API_URL}/api/users`, {
      data: {},
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.id).toBeDefined();
    expect(data.id.length).toBeGreaterThan(0);
  });
});

test.describe("Subscription API", () => {
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
});
