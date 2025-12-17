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
use allframe::serde_json;
use allframe::tokio;
use allframe::tokio::net::TcpListener;
use anyhow::Result;
use api::{create_router, route_to_handler, AppState, Config};
use bytes::Bytes;
use http_body_util::Full;
use hyper_util::rt::TokioIo;
use tracing::info;

/// HTTP request handler that routes to AllFrame handlers
async fn handle_request(
    router: Arc<Router>,
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let method = req.method().as_str();
    let path = req.uri().path();

    info!("Incoming request: {} {}", method, path);

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

    // Route to appropriate handler
    let response_body = match route_to_handler(method, path) {
        Some(handler_name) => {
            match router.execute(handler_name).await {
                Ok(body) => body,
                Err(e) => {
                    tracing::error!("Handler error: {}", e);
                    serde_json::json!({"error": "Internal server error"}).to_string()
                }
            }
        }
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(
                    serde_json::json!({"error": "Not found"}).to_string(),
                )))
                .unwrap());
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .body(Full::new(Bytes::from(response_body)))
        .unwrap())
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
    let router = Arc::new(create_router(app_state));

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

        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let router = router.clone();
                handle_request(router, req)
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
