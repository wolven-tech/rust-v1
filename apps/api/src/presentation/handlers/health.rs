use axum::{extract::State, Json};
use chrono::Utc;
use tracing::instrument;

use crate::{infrastructure::state::AppState, presentation::dto::subscribe::HealthResponse};

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    )
)]
#[instrument(skip(_state))]
pub async fn health_check(State(_state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
