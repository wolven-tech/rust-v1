/**
 * AllFrame API Client
 *
 * This is the infrastructure layer for making HTTP requests to the AllFrame API.
 * It handles the low-level details of HTTP communication and provides a clean
 * interface for the application layer (server actions).
 */

import type {
  CalculateShippingRequest,
  CreateOrderRequest,
  GetUserRequest,
  Metrics,
  Order,
  SearchProductsRequest,
  SearchProductsResponse,
  ShippingCost,
  User,
} from "@/domain/api";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4400";

class ApiError extends Error {
  constructor(
    message: string,
    public statusCode?: number,
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function request<T>(
  endpoint: string,
  options: RequestInit = {},
): Promise<T> {
  const url = `${API_URL}${endpoint}`;

  const response = await fetch(url, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new ApiError(
      error.error || `Request failed with status ${response.status}`,
      response.status,
    );
  }

  return response.json();
}

export const apiClient = {
  /**
   * Search for products by query
   */
  searchProducts: (
    data: SearchProductsRequest,
  ): Promise<SearchProductsResponse> =>
    request("/api/products/search", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  /**
   * Create a new order
   */
  createOrder: (data: CreateOrderRequest): Promise<Order> =>
    request("/api/orders", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  /**
   * Calculate shipping cost
   */
  calculateShipping: (data: CalculateShippingRequest): Promise<ShippingCost> =>
    request("/api/shipping/calculate", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  /**
   * Get user information
   */
  getUser: (data: GetUserRequest): Promise<User> =>
    request("/api/users", {
      method: "POST",
      body: JSON.stringify(data),
    }),

  /**
   * Get dashboard metrics
   */
  getMetrics: (): Promise<Metrics> =>
    request("/api/metrics", {
      method: "GET",
    }),
};

export type { ApiError };
