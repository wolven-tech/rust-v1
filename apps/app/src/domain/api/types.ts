/**
 * Domain types for the AllFrame API
 *
 * These types represent the core business entities returned by the API.
 * They are framework-agnostic and can be used across the application.
 */

// Products
export interface Product {
  id: string;
  name: string;
}

export interface SearchProductsResponse {
  query: string;
  results: Product[];
}

export interface SearchProductsRequest {
  query: string;
}

// Orders
export interface Order {
  order_id: string;
  product: string;
  status: string;
}

export interface CreateOrderRequest {
  product: string;
  quantity: number;
}

// Shipping
export interface ShippingCost {
  weight: number;
  cost: number;
}

export interface CalculateShippingRequest {
  weight: number;
}

// Users
export interface User {
  id: string;
  name: string;
  email: string;
}

export interface GetUserRequest {
  user_id?: string;
}

// Metrics
export interface Metrics {
  products: number;
  orders: number;
  users: number;
  api_calls: number;
}
