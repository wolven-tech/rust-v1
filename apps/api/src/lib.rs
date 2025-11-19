//! V1 API Library
//!
//! This library provides the core functionality for the V1 API service,
//! implementing Clean Architecture patterns.

use axum::{
    routing::{get, post},
    Router,
};
use std::time::Duration;
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
    ),
    components(
        schemas(
            presentation::dto::subscribe::SubscribeRequest,
            presentation::dto::subscribe::SubscribeResponse,
            presentation::dto::subscribe::HealthResponse,
            presentation::dto::subscribe::RootResponse,
        )
    ),
    tags(
        (name = "general", description = "General endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "subscription", description = "Newsletter subscription"),
    ),
    info(
        title = "V1 API",
        version = "1.0.0",
        description = "API for V1 application",
    )
)]
struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Root endpoint
        .route("/", get(presentation::handlers::root::root))
        // Health check
        .route("/health", get(presentation::handlers::health::health_check))
        // API routes
        .route(
            "/api/subscribe",
            post(presentation::handlers::subscribe::subscribe),
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
