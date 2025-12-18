//! AllFrame API Client
//!
//! This module provides an HTTP client for communicating with the AllFrame API.
//! It supports product search, order creation, shipping calculation, and user
//! management.

use allframe::reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::error::ApiError;

/// AllFrame API client configuration
#[derive(Debug, Clone)]
pub struct AllFrameClientConfig {
    /// Base URL for the AllFrame API
    pub base_url: String,
    /// Optional API key for authentication
    pub api_key: Option<String>,
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

impl Default for AllFrameClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:4400".to_string(),
            api_key: None,
            timeout_secs: 30,
        }
    }
}

impl AllFrameClientConfig {
    pub fn from_env() -> Self {
        Self {
            base_url: std::env::var("ALLFRAME_API_URL")
                .unwrap_or_else(|_| "http://localhost:4400".to_string()),
            api_key: std::env::var("ALLFRAME_API_KEY").ok(),
            timeout_secs: std::env::var("ALLFRAME_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}

/// AllFrame API client
pub struct AllFrameClient {
    client: Client,
    config: AllFrameClientConfig,
}

// Request/Response types for AllFrame API
#[derive(Debug, Serialize)]
pub struct SearchProductsRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub price: Option<f64>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchProductsResponse {
    pub query: String,
    pub results: Vec<Product>,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderRequest {
    pub product: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub product: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct CalculateShippingRequest {
    pub weight: f64,
}

#[derive(Debug, Deserialize)]
pub struct CalculateShippingResponse {
    pub weight: f64,
    pub cost: f64,
}

#[derive(Debug, Serialize)]
pub struct GetUserRequest {
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl AllFrameClient {
    /// Create a new AllFrame client with the given configuration
    pub fn new(config: AllFrameClientConfig) -> Result<Self, ApiError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| {
                ApiError::external_service_error(format!("Failed to create HTTP client: {}", e))
            })?;

        Ok(Self { client, config })
    }

    /// Create a new AllFrame client with default configuration from environment
    pub fn from_env() -> Result<Self, ApiError> {
        Self::new(AllFrameClientConfig::from_env())
    }

    /// Search for products
    #[instrument(skip(self))]
    pub async fn search_products(&self, query: String) -> Result<SearchProductsResponse, ApiError> {
        info!("Searching products with query: {}", query);

        let url = format!("{}/api/products/search", self.config.base_url);
        let request = SearchProductsRequest { query };

        let mut req = self.client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req
            .send()
            .await
            .map_err(|e| ApiError::external_service_error(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ApiError::external_service_error(format!(
                "AllFrame API returned error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(|e| {
            ApiError::external_service_error(format!("Failed to parse response: {}", e))
        })
    }

    /// Create an order
    #[instrument(skip(self))]
    pub async fn create_order(
        &self,
        product: String,
        quantity: u32,
    ) -> Result<CreateOrderResponse, ApiError> {
        info!(
            "Creating order for product: {}, quantity: {}",
            product, quantity
        );

        let url = format!("{}/api/orders", self.config.base_url);
        let request = CreateOrderRequest { product, quantity };

        let mut req = self.client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req
            .send()
            .await
            .map_err(|e| ApiError::external_service_error(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ApiError::external_service_error(format!(
                "AllFrame API returned error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(|e| {
            ApiError::external_service_error(format!("Failed to parse response: {}", e))
        })
    }

    /// Calculate shipping cost
    #[instrument(skip(self))]
    pub async fn calculate_shipping(
        &self,
        weight: f64,
    ) -> Result<CalculateShippingResponse, ApiError> {
        info!("Calculating shipping for weight: {}", weight);

        let url = format!("{}/api/shipping/calculate", self.config.base_url);
        let request = CalculateShippingRequest { weight };

        let mut req = self.client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req
            .send()
            .await
            .map_err(|e| ApiError::external_service_error(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ApiError::external_service_error(format!(
                "AllFrame API returned error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(|e| {
            ApiError::external_service_error(format!("Failed to parse response: {}", e))
        })
    }

    /// Get user information
    #[instrument(skip(self))]
    pub async fn get_user(&self, user_id: Option<String>) -> Result<UserResponse, ApiError> {
        info!("Getting user: {:?}", user_id);

        let url = format!("{}/api/users", self.config.base_url);
        let request = GetUserRequest { user_id };

        let mut req = self.client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req
            .send()
            .await
            .map_err(|e| ApiError::external_service_error(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(ApiError::external_service_error(format!(
                "AllFrame API returned error: {}",
                response.status()
            )));
        }

        response.json().await.map_err(|e| {
            ApiError::external_service_error(format!("Failed to parse response: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{body_json, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::*;

    #[tokio::test]
    async fn test_search_products_success() {
        // Arrange
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "query": "widget",
            "results": [
                {"id": "1", "name": "Widget A"},
                {"id": "2", "name": "Widget B"}
            ]
        });

        Mock::given(method("POST"))
            .and(path("/api/products/search"))
            .and(body_json(serde_json::json!({"query": "widget"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: None,
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.search_products("widget".to_string()).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.query, "widget");
        assert_eq!(response.results.len(), 2);
        assert_eq!(response.results[0].name, "Widget A");
    }

    #[tokio::test]
    async fn test_search_products_server_error() {
        // Arrange
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/products/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: None,
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.search_products("test".to_string()).await;

        // Assert
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_order_success() {
        // Arrange
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "order_id": "order-123",
            "product": "Widget",
            "status": "created"
        });

        Mock::given(method("POST"))
            .and(path("/api/orders"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: None,
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.create_order("Widget".to_string(), 2).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.order_id, "order-123");
        assert_eq!(response.product, "Widget");
        assert_eq!(response.status, "created");
    }

    #[tokio::test]
    async fn test_calculate_shipping_success() {
        // Arrange
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "weight": 10.0,
            "cost": 30.0
        });

        Mock::given(method("POST"))
            .and(path("/api/shipping/calculate"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: None,
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.calculate_shipping(10.0).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.weight, 10.0);
        assert_eq!(response.cost, 30.0);
    }

    #[tokio::test]
    async fn test_get_user_success() {
        // Arrange
        let mock_server = MockServer::start().await;

        let response_body = serde_json::json!({
            "id": "user-123",
            "name": "John Doe",
            "email": "john@example.com"
        });

        Mock::given(method("POST"))
            .and(path("/api/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: None,
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.get_user(Some("user-123".to_string())).await;

        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, "user-123");
        assert_eq!(response.name, "John Doe");
        assert_eq!(response.email, "john@example.com");
    }

    #[tokio::test]
    async fn test_client_with_api_key() {
        // Arrange
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/products/search"))
            .and(wiremock::matchers::header(
                "Authorization",
                "Bearer test-api-key",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "query": "test",
                "results": []
            })))
            .mount(&mock_server)
            .await;

        let config = AllFrameClientConfig {
            base_url: mock_server.uri(),
            api_key: Some("test-api-key".to_string()),
            timeout_secs: 5,
        };
        let client = AllFrameClient::new(config).unwrap();

        // Act
        let result = client.search_products("test".to_string()).await;

        // Assert
        assert!(result.is_ok());
    }
}
