use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub product: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub product: String,
    pub status: String,
}
