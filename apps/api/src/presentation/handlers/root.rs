use axum::Json;
use tracing::instrument;

use crate::presentation::dto::subscribe::RootResponse;

/// Root endpoint
#[utoipa::path(
    get,
    path = "/",
    tag = "general",
    responses(
        (status = 200, description = "API is running", body = RootResponse),
    )
)]
#[instrument]
pub async fn root() -> Json<RootResponse> {
    Json(RootResponse {
        message: "V1 API is running".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
