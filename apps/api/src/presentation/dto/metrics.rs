use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub products: u32,
    pub orders: u32,
    pub users: u32,
    pub api_calls: u32,
}
