//! V1 API Library
//!
//! This library provides the core functionality for the V1 API service,
//! implementing Clean Architecture patterns with AllFrame e-commerce integration.

use std::time::Duration;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod presentation;

// Re-export commonly used types
pub use config::Config;
pub use infrastructure::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        presentation::handlers::root::root,
        presentation::handlers::health::health_check,
        presentation::handlers::subscribe::subscribe,
        presentation::handlers::allframe::search_products,
        presentation::handlers::allframe::create_order,
        presentation::handlers::allframe::calculate_shipping,
        presentation::handlers::allframe::get_user,
    ),
    components(
        schemas(
            presentation::dto::subscribe::SubscribeRequest,
            presentation::dto::subscribe::SubscribeResponse,
            presentation::dto::subscribe::HealthResponse,
            presentation::dto::subscribe::RootResponse,
            presentation::dto::allframe::SearchProductsRequest,
            presentation::dto::allframe::SearchProductsResponse,
            presentation::dto::allframe::Product,
            presentation::dto::allframe::CreateOrderRequest,
            presentation::dto::allframe::CreateOrderResponse,
            presentation::dto::allframe::CalculateShippingRequest,
            presentation::dto::allframe::CalculateShippingResponse,
            presentation::dto::allframe::GetUserRequest,
            presentation::dto::allframe::UserResponse,
        )
    ),
    tags(
        (name = "general", description = "General endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "subscription", description = "Newsletter subscription"),
        (name = "allframe", description = "AllFrame e-commerce endpoints"),
    ),
    info(
        title = "V1 API",
        version = "1.0.0",
        description = "API for V1 application with AllFrame e-commerce integration",
    )
)]
struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Root endpoint
        .route("/", get(presentation::handlers::root::root))
        // Health check
        .route("/health", get(presentation::handlers::health::health_check))
        // API routes - Subscription
        .route(
            "/api/subscribe",
            post(presentation::handlers::subscribe::subscribe),
        )
        // API routes - AllFrame e-commerce
        .route(
            "/api/allframe/products/search",
            post(presentation::handlers::allframe::search_products),
        )
        .route(
            "/api/allframe/orders",
            post(presentation::handlers::allframe::create_order),
        )
        .route(
            "/api/allframe/shipping/calculate",
            post(presentation::handlers::allframe::calculate_shipping),
        )
        .route(
            "/api/allframe/users",
            post(presentation::handlers::allframe::get_user),
        )
        // API documentation
        .merge(Scalar::with_url("/api/docs", ApiDoc::openapi()))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        // Application state
        .with_state(state)
}
