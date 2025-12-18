//! Integration tests for the V1 API
//!
//! These tests use AllFrame's router directly to test handlers.

use std::sync::Arc;

use api::{create_router, AppState, Config};

#[tokio::test]
async fn test_health_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("health").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("\"status\":\"ok\""));
    assert!(body.contains("\"version\""));
}

#[tokio::test]
async fn test_root_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("root").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("V1 API is running"));
}

#[tokio::test]
async fn test_search_products_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("search_products").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("results"));
}

#[tokio::test]
async fn test_create_order_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("create_order").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("order_id"));
    assert!(body.contains("created"));
}

#[tokio::test]
async fn test_calculate_shipping_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("calculate_shipping").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("weight"));
    assert!(body.contains("cost"));
}

#[tokio::test]
async fn test_get_user_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("get_user").await;
    assert!(response.is_ok());

    let body = response.unwrap();
    assert!(body.contains("id"));
    assert!(body.contains("name"));
    assert!(body.contains("email"));
}

#[tokio::test]
async fn test_handler_count() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    // Should have 7 handlers registered
    assert_eq!(router.handlers_count(), 7);
}

#[tokio::test]
async fn test_nonexistent_handler() {
    let config = Config::default();
    let state = Arc::new(AppState::new(config));
    let router = create_router(state);

    let response = router.execute("nonexistent").await;
    assert!(response.is_err());
}
