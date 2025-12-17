use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SearchProductsRequest {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct SearchProductsResponse {
    pub query: String,
    pub results: Vec<Product>,
}
