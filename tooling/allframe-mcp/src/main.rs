//! AllFrame MCP Server
//!
//! This MCP server exposes AllFrame e-commerce tools to Claude.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p allframe-mcp
//! ```
//!
//! # Debug Mode
//!
//! ```bash
//! ALLFRAME_MCP_DEBUG=1 cargo run -p allframe-mcp
//! ```

use allframe_core::router::Router;
use allframe_mcp::{init_tracing, McpServer, StdioConfig, StdioTransport};

#[tokio::main]
async fn main() {
    // Initialize tracing
    init_tracing();

    // Create AllFrame router with e-commerce handlers
    let router = create_router();

    // Create MCP server
    let mcp = McpServer::new(router);

    // Configure the stdio transport
    let config = StdioConfig::default()
        .with_debug_tool(true)
        .with_server_name("allframe-mcp");

    // Run the server
    StdioTransport::new(mcp, config).serve().await;
}

/// Create router with e-commerce handlers
fn create_router() -> Router {
    let mut router = Router::new();

    // Get user information
    router.register("get_user", || async {
        let user_id = uuid::Uuid::new_v4();
        format!(
            r#"{{"id": "{}", "name": "User {}", "email": "user@example.com"}}"#,
            user_id, user_id
        )
    });

    // Create an order
    router.register("create_order", || async {
        let order_id = uuid::Uuid::new_v4();
        format!(
            r#"{{"order_id": "{}", "product": "Widget", "status": "created"}}"#,
            order_id
        )
    });

    // Search products
    router.register("search_products", || async {
        r#"{"query": "search", "results": [{"id": "1", "name": "Product A"}, {"id": "2", "name": "Product B"}]}"#.to_string()
    });

    // Calculate shipping
    router.register("calculate_shipping", || async {
        let weight = 10.0;
        let cost = weight * 3.0;
        format!(r#"{{"weight": {}, "cost": {:.2}}}"#, weight, cost)
    });

    router
}
