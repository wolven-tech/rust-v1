use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SubscribeRequest {
    pub email: String,
    pub user_group: String,
}

#[derive(Debug, Serialize)]
pub struct SubscribeResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct RootResponse {
    pub message: String,
    pub version: String,
}
