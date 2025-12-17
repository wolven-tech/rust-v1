use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CalculateShippingRequest {
    pub weight: f64,
}

#[derive(Debug, Serialize)]
pub struct CalculateShippingResponse {
    pub weight: f64,
    pub cost: f64,
}
