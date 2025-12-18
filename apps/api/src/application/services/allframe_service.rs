use std::sync::atomic::{AtomicU32, Ordering};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    error::ApiError,
    presentation::dto::{
        metrics::MetricsResponse,
        orders::CreateOrderResponse,
        products::{Product, SearchProductsResponse},
        shipping::CalculateShippingResponse,
        users::UserResponse,
    },
};

/// AllFrame e-commerce service
///
/// This service provides e-commerce functionality including:
/// - Product search
/// - Order creation
/// - Shipping calculation
/// - User management
///
/// Tracks metrics dynamically using atomic counters for thread-safety.
pub struct AllFrameService {
    /// Number of orders created
    orders_count: AtomicU32,
    /// Number of users registered
    users_count: AtomicU32,
    /// Number of API calls made
    api_calls: AtomicU32,
}

impl AllFrameService {
    pub fn new() -> Self {
        Self {
            orders_count: AtomicU32::new(0),
            users_count: AtomicU32::new(0),
            api_calls: AtomicU32::new(0),
        }
    }

    /// Increment API calls counter
    fn track_api_call(&self) {
        self.api_calls.fetch_add(1, Ordering::Relaxed);
    }

    /// Search for products by query
    #[instrument(skip(self))]
    pub async fn search_products(&self, query: String) -> Result<SearchProductsResponse, ApiError> {
        self.track_api_call();
        info!("Searching products with query: {}", query);

        // Mock product catalog
        let catalog = vec![
            Product { id: "p1".to_string(), name: "Widget Pro".to_string() },
            Product { id: "p2".to_string(), name: "Widget Basic".to_string() },
            Product { id: "p3".to_string(), name: "Gadget X100".to_string() },
            Product { id: "p4".to_string(), name: "Gadget Mini".to_string() },
            Product { id: "p5".to_string(), name: "Smart Sensor".to_string() },
            Product { id: "p6".to_string(), name: "Power Module".to_string() },
            Product { id: "p7".to_string(), name: "Control Board".to_string() },
            Product { id: "p8".to_string(), name: "Display Panel".to_string() },
            Product { id: "p9".to_string(), name: "Cable Kit".to_string() },
            Product { id: "p10".to_string(), name: "Battery Pack".to_string() },
        ];

        // Filter products based on query (case-insensitive)
        let query_lower = query.to_lowercase();
        let results: Vec<Product> = catalog
            .into_iter()
            .filter(|p| p.name.to_lowercase().contains(&query_lower))
            .collect();

        Ok(SearchProductsResponse { query, results })
    }

    /// Create a new order
    #[instrument(skip(self))]
    pub async fn create_order(
        &self,
        product: String,
        _quantity: u32,
    ) -> Result<CreateOrderResponse, ApiError> {
        self.track_api_call();
        info!("Creating order for product: {}", product);

        // Increment order count
        self.orders_count.fetch_add(1, Ordering::Relaxed);

        // Simulated AllFrame response
        let order_id = Uuid::new_v4().to_string();

        Ok(CreateOrderResponse {
            order_id,
            product,
            status: "created".to_string(),
        })
    }

    /// Calculate shipping cost based on weight
    #[instrument(skip(self))]
    pub async fn calculate_shipping(
        &self,
        weight: f64,
    ) -> Result<CalculateShippingResponse, ApiError> {
        self.track_api_call();
        info!("Calculating shipping for weight: {}", weight);

        if weight <= 0.0 {
            return Err(ApiError::bad_request("Weight must be positive"));
        }

        // Simulated AllFrame response: $3 per unit weight
        let cost = weight * 3.0;

        Ok(CalculateShippingResponse { weight, cost })
    }

    /// Get user information (or register a new user if no ID provided)
    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: Option<String>) -> Result<UserResponse, ApiError> {
        self.track_api_call();
        info!("Getting user: {:?}", user_id);

        // If no user_id provided, this is a new user registration
        let (id, is_new) = match user_id {
            Some(existing_id) => (existing_id, false),
            None => (Uuid::new_v4().to_string(), true),
        };

        // Increment user count for new registrations
        if is_new {
            self.users_count.fetch_add(1, Ordering::Relaxed);
        }

        Ok(UserResponse {
            id: id.clone(),
            name: format!("User {}", id),
            email: "user@example.com".to_string(),
        })
    }

    /// Get dashboard metrics
    #[instrument(skip(self))]
    pub async fn get_metrics(&self) -> Result<MetricsResponse, ApiError> {
        info!("Getting dashboard metrics");

        // Dynamic metrics from atomic counters
        Ok(MetricsResponse {
            products: 10, // Static product catalog size
            orders: self.orders_count.load(Ordering::Relaxed),
            users: self.users_count.load(Ordering::Relaxed),
            api_calls: self.api_calls.load(Ordering::Relaxed),
        })
    }
}

impl Default for AllFrameService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_products() {
        let service = AllFrameService::new();
        let result = service.search_products("Widget".to_string()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.query, "Widget");
        assert!(!response.results.is_empty());
    }

    #[tokio::test]
    async fn test_create_order() {
        let service = AllFrameService::new();
        let result = service.create_order("Widget".to_string(), 1).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.product, "Widget");
        assert_eq!(response.status, "created");
    }

    #[tokio::test]
    async fn test_calculate_shipping() {
        let service = AllFrameService::new();
        let result = service.calculate_shipping(10.0).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.weight, 10.0);
        assert_eq!(response.cost, 30.0);
    }

    #[tokio::test]
    async fn test_calculate_shipping_invalid_weight() {
        let service = AllFrameService::new();
        let result = service.calculate_shipping(-1.0).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user() {
        let service = AllFrameService::new();
        let result = service.get_user(None).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.id.is_empty());
        assert_eq!(response.email, "user@example.com");
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let service = AllFrameService::new();

        // Initial metrics should be zero
        let metrics = service.get_metrics().await.unwrap();
        assert_eq!(metrics.orders, 0);
        assert_eq!(metrics.users, 0);
        assert_eq!(metrics.api_calls, 0);

        // Create some orders
        service.create_order("Widget".to_string(), 1).await.unwrap();
        service.create_order("Gadget".to_string(), 2).await.unwrap();

        // Register a new user
        service.get_user(None).await.unwrap();

        // Search for products
        service.search_products("Widget".to_string()).await.unwrap();

        // Check metrics
        let metrics = service.get_metrics().await.unwrap();
        assert_eq!(metrics.orders, 2);
        assert_eq!(metrics.users, 1);
        assert_eq!(metrics.api_calls, 4); // 2 orders + 1 user + 1 search
    }
}
