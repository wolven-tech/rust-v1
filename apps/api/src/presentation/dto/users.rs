use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetUserRequest {
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
