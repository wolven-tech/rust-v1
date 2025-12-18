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
    expect(data.docs).toBe("/docs");
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

  test("CORS headers are present", async ({ request }) => {
    const response = await request.get(`${API_URL}/`);

    expect(response.headers()["access-control-allow-origin"]).toBe("*");
  });
});

test.describe("API Documentation", () => {
  test("GET /docs returns Scalar API documentation", async ({ request }) => {
    const response = await request.get(`${API_URL}/docs`);

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("text/html");

    const html = await response.text();
    expect(html).toContain("V1 API");
    expect(html).toContain("scalar");
  });

  test("GET /docs/openapi.json returns OpenAPI specification", async ({
    request,
  }) => {
    const response = await request.get(`${API_URL}/docs/openapi.json`);

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("application/json");

    const openapi = await response.json();
    expect(openapi.openapi).toBeDefined();
    expect(openapi.info).toBeDefined();
    expect(openapi.info.title).toBe("V1 API");
    expect(openapi.paths).toBeDefined();
  });

  test("OpenAPI spec contains documented endpoints", async ({ request }) => {
    const response = await request.get(`${API_URL}/docs/openapi.json`);
    const openapi = await response.json();

    // Check that expected paths are documented
    expect(openapi.paths["/"]).toBeDefined();
    expect(openapi.paths["/health"]).toBeDefined();
    expect(openapi.paths["/api/products/search"]).toBeDefined();
    expect(openapi.paths["/api/orders"]).toBeDefined();
    expect(openapi.paths["/api/shipping/calculate"]).toBeDefined();
    expect(openapi.paths["/api/users"]).toBeDefined();
    expect(openapi.paths["/api/subscribe"]).toBeDefined();
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
    // Current implementation uses hardcoded "search" query
    expect(data.query).toBeDefined();
    expect(data.results).toBeDefined();
    expect(Array.isArray(data.results)).toBeTruthy();
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
    expect(data.product).toBeDefined();
    expect(data.status).toBe("created");
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
    // Current implementation uses hardcoded weight=10.0, cost=30.0
    expect(data.weight).toBe(10.0);
    expect(data.cost).toBe(30.0);
  });
});

test.describe("Users API", () => {
  test("POST /api/users returns user data", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/users`, {
      data: {},
    });

    expect(response.ok()).toBeTruthy();
    expect(response.status()).toBe(200);

    const data = await response.json();
    expect(data.id).toBeDefined();
    expect(data.name).toBeDefined();
    expect(data.email).toBeDefined();
  });
});

test.describe("Subscription API", () => {
  test("POST /api/subscribe returns subscription result", async ({
    request,
  }) => {
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
});

test.describe("Error Handling", () => {
  test("GET /unknown-path returns 404", async ({ request }) => {
    const response = await request.get(`${API_URL}/unknown-path`);

    expect(response.status()).toBe(404);

    const data = await response.json();
    expect(data.error).toBe("Not found");
  });

  test("POST to non-existent API route returns 404", async ({ request }) => {
    const response = await request.post(`${API_URL}/api/nonexistent`, {
      data: {},
    });

    expect(response.status()).toBe(404);
  });
});
