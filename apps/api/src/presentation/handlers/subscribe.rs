use axum::{extract::State, Json};
use tracing::{info, instrument};

use crate::{
    error::ApiError,
    infrastructure::state::AppState,
    presentation::dto::subscribe::{SubscribeRequest, SubscribeResponse},
};

/// Subscribe to newsletter
#[utoipa::path(
    post,
    path = "/api/subscribe",
    tag = "subscription",
    request_body = SubscribeRequest,
    responses(
        (status = 200, description = "Subscription processed", body = SubscribeResponse),
        (status = 400, description = "Invalid request"),
        (status = 502, description = "External service error"),
    )
)]
#[instrument(skip(state))]
pub async fn subscribe(
    State(state): State<AppState>,
    Json(request): Json<SubscribeRequest>,
) -> Result<Json<SubscribeResponse>, ApiError> {
    info!("Processing subscription for email: {}", request.email);

    let (success, message, id) = state
        .subscription_service()
        .subscribe(request.email, request.user_group)
        .await?;

    Ok(Json(SubscribeResponse {
        success,
        message,
        id,
    }))
}
