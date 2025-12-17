//! V1 API Library
//!
//! This library provides the core functionality for the V1 API service,
//! powered by AllFrame's protocol-agnostic router.

use std::sync::Arc;

use allframe::chrono;
use allframe::router::{rest::RestAdapter, Router};
use allframe::serde_json::{self, json};

pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod presentation;

// Re-export commonly used types
pub use config::Config;
pub use infrastructure::AppState;

/// Create the AllFrame router with all handlers registered
pub fn create_router(state: Arc<AppState>) -> Router {
    let mut router = Router::new();

    // Add REST adapter
    let adapter = RestAdapter::new();
    router.add_adapter(Box::new(adapter));

    // Root endpoint
    router.register("root", || async move {
        json!({
            "message": "V1 API is running",
            "version": env!("CARGO_PKG_VERSION")
        })
        .to_string()
    });

    // Health check
    router.register("health", || async move {
        json!({
            "status": "ok",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION")
        })
        .to_string()
    });

    // Products search
    let products_state = state.clone();
    router.register("search_products", move || {
        let state = products_state.clone();
        async move {
            match state.allframe_service().search_products("search".to_string()).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(_) => json!({"error": "Failed to search products"}).to_string(),
            }
        }
    });

    // Create order
    let orders_state = state.clone();
    router.register("create_order", move || {
        let state = orders_state.clone();
        async move {
            match state.allframe_service().create_order("Widget".to_string(), 1).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(_) => json!({"error": "Failed to create order"}).to_string(),
            }
        }
    });

    // Calculate shipping
    let shipping_state = state.clone();
    router.register("calculate_shipping", move || {
        let state = shipping_state.clone();
        async move {
            match state.allframe_service().calculate_shipping(10.0).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(_) => json!({"error": "Failed to calculate shipping"}).to_string(),
            }
        }
    });

    // Get user
    let users_state = state.clone();
    router.register("get_user", move || {
        let state = users_state.clone();
        async move {
            match state.allframe_service().get_user(None).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(_) => json!({"error": "Failed to get user"}).to_string(),
            }
        }
    });

    // Subscribe
    let subscribe_state = state.clone();
    router.register("subscribe", move || {
        let state = subscribe_state.clone();
        async move {
            match state
                .subscription_service()
                .subscribe("test@example.com".to_string(), "default".to_string())
                .await
            {
                Ok((success, message, id)) => {
                    json!({
                        "success": success,
                        "message": message,
                        "id": id
                    })
                    .to_string()
                }
                Err(_) => json!({"success": false, "error": "Failed to subscribe"}).to_string(),
            }
        }
    });

    router
}

/// Route path to handler name mapping
pub fn route_to_handler(method: &str, path: &str) -> Option<&'static str> {
    match (method, path) {
        ("GET", "/") => Some("root"),
        ("GET", "/health") => Some("health"),
        ("POST", "/api/products/search") => Some("search_products"),
        ("POST", "/api/orders") => Some("create_order"),
        ("POST", "/api/shipping/calculate") => Some("calculate_shipping"),
        ("POST", "/api/users") => Some("get_user"),
        ("POST", "/api/subscribe") => Some("subscribe"),
        _ => None,
    }
}
