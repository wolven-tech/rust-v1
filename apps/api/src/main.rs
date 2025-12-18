//! V1 API Server
//!
//! HTTP server powered by AllFrame's router and hyper.

use std::net::SocketAddr;
use std::sync::Arc;

use allframe::hyper;
use allframe::hyper::body::Incoming;
use allframe::hyper::server::conn::http1;
use allframe::hyper::service::service_fn;
use allframe::hyper::{Request, Response, StatusCode};
use allframe::router::Router;
use allframe::serde_json::{self, json};
use allframe::tokio;
use allframe::tokio::net::TcpListener;
use anyhow::Result;
use api::{create_router, route_to_handler, AppState, Config};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper_util::rt::TokioIo;
use tracing::info;

/// Helper to create CORS response
fn cors_response(status: StatusCode, body: String) -> Response<Full<Bytes>> {
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

/// HTTP request handler that routes to AllFrame handlers
async fn handle_request(
    router: Arc<Router>,
    state: Arc<AppState>,
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().as_str().to_string();
    let path = req.uri().path().to_string();

    info!("Incoming request: {} {}", method, path);

    // Handle CORS preflight requests
    if method == "OPTIONS" {
        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
            .header("Access-Control-Max-Age", "86400")
            .body(Full::new(Bytes::new()))
            .unwrap());
    }

    // Handle documentation routes specially
    if path == "/docs" {
        let html = router.scalar("V1 API", env!("CARGO_PKG_VERSION"));
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Access-Control-Allow-Origin", "*")
            .body(Full::new(Bytes::from(html)))
            .unwrap());
    }

    if path == "/docs/openapi.json" {
        let openapi = router.openapi_json("V1 API", env!("CARGO_PKG_VERSION"));
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(Full::new(Bytes::from(openapi)))
            .unwrap());
    }

    // Read request body for POST requests
    let body_bytes = match req.into_body().collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => Bytes::new(),
    };

    // Handle routes that need request body data
    let response_body = match (method.as_str(), path.as_str()) {
        ("POST", "/api/products/search") => {
            let query = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                json.get("query")
                    .and_then(|v| v.as_str())
                    .unwrap_or("search")
                    .to_string()
            } else {
                "search".to_string()
            };
            match state.allframe_service().search_products(query).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(e) => json!({"error": e.to_string()}).to_string(),
            }
        }
        ("POST", "/api/orders") => {
            let (product, quantity) = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                let product = json.get("product")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Widget")
                    .to_string();
                let quantity = json.get("quantity")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as u32;
                (product, quantity)
            } else {
                ("Widget".to_string(), 1)
            };
            match state.allframe_service().create_order(product, quantity).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(e) => json!({"error": e.to_string()}).to_string(),
            }
        }
        ("POST", "/api/shipping/calculate") => {
            let weight = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                json.get("weight")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            match state.allframe_service().calculate_shipping(weight).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(e) => return Ok(cors_response(StatusCode::BAD_REQUEST, json!({"error": e.to_string()}).to_string())),
            }
        }
        ("POST", "/api/users") => {
            let user_id = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                json.get("user_id")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
            } else {
                None
            };
            match state.allframe_service().get_user(user_id).await {
                Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(e) => json!({"error": e.to_string()}).to_string(),
            }
        }
        ("POST", "/api/subscribe") => {
            let (email, user_group) = if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
                let email = json.get("email")
                    .and_then(|v| v.as_str())
                    .unwrap_or("test@example.com")
                    .to_string();
                let user_group = json.get("userGroup")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                (email, user_group)
            } else {
                ("test@example.com".to_string(), "default".to_string())
            };
            match state.subscription_service().subscribe(email, user_group).await {
                Ok((success, message, id)) => {
                    json!({
                        "success": success,
                        "message": message,
                        "id": id
                    })
                    .to_string()
                }
                Err(_) => json!({"success": false, "error": "Failed to subscribe"}).to_string(),
            }
        }
        ("GET", "/api/metrics") => {
            // Return dashboard metrics
            match state.allframe_service().get_metrics().await {
                Ok(metrics) => serde_json::to_string(&metrics).unwrap_or_else(|_| {
                    json!({"error": "Failed to serialize response"}).to_string()
                }),
                Err(e) => json!({"error": e.to_string()}).to_string(),
            }
        }
        _ => {
            // Fall back to router for other routes (GET requests, etc.)
            match route_to_handler(&method, &path) {
                Some(handler_name) => {
                    match router.execute(handler_name).await {
                        Ok(body) => body,
                        Err(e) => {
                            tracing::error!("Handler error: {}", e);
                            json!({"error": "Internal server error"}).to_string()
                        }
                    }
                }
                None => {
                    return Ok(cors_response(
                        StatusCode::NOT_FOUND,
                        json!({"error": "Not found"}).to_string(),
                    ));
                }
            }
        }
    };

    Ok(cors_response(StatusCode::OK, response_body))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file first so RUST_LOG is available for tracing
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,allframe=debug".into()),
        )
        .with_target(false)
        .init();

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting V1 API server with config: {:?}", config);

    // Create application state
    let app_state = Arc::new(AppState::new(config.clone()));

    // Build AllFrame router
    let router = Arc::new(create_router(app_state.clone()));

    info!(
        "AllFrame router initialized with {} handlers",
        router.handlers_count()
    );

    // Start HTTP server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = TcpListener::bind(addr).await?;
    let local_addr = listener.local_addr()?;

    // Print startup info
    println!("\n🚀 V1 API server is running!");
    println!("   → Powered by AllFrame");
    println!("   → Local:   http://localhost:{}", local_addr.port());
    println!("   → Network: http://0.0.0.0:{}", local_addr.port());
    println!("   → Health:  http://localhost:{}/health", local_addr.port());
    println!("   → Docs:    http://localhost:{}/docs\n", local_addr.port());

    info!("V1 API server started on port {}", local_addr.port());

    // Accept connections
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let router = router.clone();
        let state = app_state.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let router = router.clone();
                let state = state.clone();
                handle_request(router, state, req)
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .await
            {
                tracing::error!("Error serving connection: {:?}", err);
            }
        });
    }
}
