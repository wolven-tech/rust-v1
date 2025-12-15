use axum::{extract::State, Json};
use tracing::{info, instrument};

use crate::{
    error::ApiError,
    infrastructure::state::AppState,
    presentation::dto::allframe::{
        CalculateShippingRequest, CalculateShippingResponse, CreateOrderRequest,
        CreateOrderResponse, GetUserRequest, SearchProductsRequest, SearchProductsResponse,
        UserResponse,
    },
};

/// Search products
#[utoipa::path(
    post,
    path = "/api/allframe/products/search",
    tag = "allframe",
    request_body = SearchProductsRequest,
    responses(
        (status = 200, description = "Products found", body = SearchProductsResponse),
        (status = 400, description = "Invalid request"),
        (status = 502, description = "External service error"),
    )
)]
#[instrument(skip(state))]
pub async fn search_products(
    State(state): State<AppState>,
    Json(request): Json<SearchProductsRequest>,
) -> Result<Json<SearchProductsResponse>, ApiError> {
    info!("Searching products with query: {}", request.query);

    let response = state
        .allframe_service()
        .search_products(request.query)
        .await?;

    Ok(Json(response))
}

/// Create order
#[utoipa::path(
    post,
    path = "/api/allframe/orders",
    tag = "allframe",
    request_body = CreateOrderRequest,
    responses(
        (status = 200, description = "Order created", body = CreateOrderResponse),
        (status = 400, description = "Invalid request"),
        (status = 502, description = "External service error"),
    )
)]
#[instrument(skip(state))]
pub async fn create_order(
    State(state): State<AppState>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>, ApiError> {
    info!(
        "Creating order for product: {}, quantity: {}",
        request.product, request.quantity
    );

    let response = state
        .allframe_service()
        .create_order(request.product, request.quantity)
        .await?;

    Ok(Json(response))
}

/// Calculate shipping
#[utoipa::path(
    post,
    path = "/api/allframe/shipping/calculate",
    tag = "allframe",
    request_body = CalculateShippingRequest,
    responses(
        (status = 200, description = "Shipping calculated", body = CalculateShippingResponse),
        (status = 400, description = "Invalid request"),
        (status = 502, description = "External service error"),
    )
)]
#[instrument(skip(state))]
pub async fn calculate_shipping(
    State(state): State<AppState>,
    Json(request): Json<CalculateShippingRequest>,
) -> Result<Json<CalculateShippingResponse>, ApiError> {
    info!("Calculating shipping for weight: {}", request.weight);

    let response = state
        .allframe_service()
        .calculate_shipping(request.weight)
        .await?;

    Ok(Json(response))
}

/// Get user
#[utoipa::path(
    post,
    path = "/api/allframe/users",
    tag = "allframe",
    request_body = GetUserRequest,
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 400, description = "Invalid request"),
        (status = 502, description = "External service error"),
    )
)]
#[instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    Json(request): Json<GetUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    info!("Getting user: {:?}", request.user_id);

    let response = state.allframe_service().get_user(request.user_id).await?;

    Ok(Json(response))
}
