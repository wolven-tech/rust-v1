use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubscribeRequest {
    #[schema(example = "user@example.com")]
    pub email: String,

    #[schema(example = "newsletter")]
    pub user_group: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RootResponse {
    pub message: String,
    pub version: String,
}
