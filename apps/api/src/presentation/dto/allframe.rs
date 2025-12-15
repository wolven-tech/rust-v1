use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// ============ Products ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct SearchProductsRequest {
    #[schema(example = "search")]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Product {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchProductsResponse {
    pub query: String,
    pub results: Vec<Product>,
}

// ============ Orders ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateOrderRequest {
    #[schema(example = "Widget")]
    pub product: String,
    #[schema(example = 1)]
    pub quantity: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub product: String,
    pub status: String,
}

// ============ Shipping ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct CalculateShippingRequest {
    #[schema(example = 10.0)]
    pub weight: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CalculateShippingResponse {
    pub weight: f64,
    pub cost: f64,
}

// ============ Users ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct GetUserRequest {
    #[schema(example = "080bc321-6f16-485a-bd45-fdc4b9a2eed6")]
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}
