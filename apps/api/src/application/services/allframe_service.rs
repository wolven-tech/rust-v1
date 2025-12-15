use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    error::ApiError,
    presentation::dto::allframe::{
        CalculateShippingResponse, CreateOrderResponse, Product, SearchProductsResponse,
        UserResponse,
    },
};

/// AllFrame e-commerce service
///
/// This service provides e-commerce functionality including:
/// - Product search
/// - Order creation
/// - Shipping calculation
/// - User management
pub struct AllFrameService {
    // In a real implementation, this would include:
    // - HTTP client for AllFrame API
    // - Configuration for API endpoints
    // - Authentication credentials
}

impl AllFrameService {
    pub fn new() -> Self {
        Self {}
    }

    /// Search for products by query
    #[instrument(skip(self))]
    pub async fn search_products(&self, query: String) -> Result<SearchProductsResponse, ApiError> {
        info!("Searching products with query: {}", query);

        // Simulated AllFrame response
        // In production, this would make an HTTP call to AllFrame API
        let results = vec![
            Product {
                id: "1".to_string(),
                name: "Product A".to_string(),
            },
            Product {
                id: "2".to_string(),
                name: "Product B".to_string(),
            },
        ];

        Ok(SearchProductsResponse { query, results })
    }

    /// Create a new order
    #[instrument(skip(self))]
    pub async fn create_order(
        &self,
        product: String,
        _quantity: u32,
    ) -> Result<CreateOrderResponse, ApiError> {
        info!("Creating order for product: {}", product);

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
        info!("Calculating shipping for weight: {}", weight);

        if weight <= 0.0 {
            return Err(ApiError::bad_request("Weight must be positive"));
        }

        // Simulated AllFrame response: $3 per unit weight
        let cost = weight * 3.0;

        Ok(CalculateShippingResponse { weight, cost })
    }

    /// Get user information
    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: Option<String>) -> Result<UserResponse, ApiError> {
        info!("Getting user: {:?}", user_id);

        // Simulated AllFrame response
        let id = user_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        Ok(UserResponse {
            id: id.clone(),
            name: format!("User {}", id),
            email: "user@example.com".to_string(),
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
        let result = service.search_products("test".to_string()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.query, "test");
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
}
