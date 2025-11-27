use std::net::SocketAddr;

use anyhow::Result;
use api::{create_router, AppState, Config};
use socket2::{Domain, Socket, Type};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file first so RUST_LOG is available for tracing
    dotenvy::dotenv().ok();

    // Initialize tracing with JSON formatting for production observability
    let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    if environment == "production" {
        // JSON format for production
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_current_span(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .init();
    } else {
        // Human-readable format for development
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
            )
            .with_target(false)
            .init();
    }

    // Load configuration
    let config = Config::from_env()?;
    info!("Starting V1 API server with config: {:?}", config);

    // Create application state
    let app_state = AppState::new(config.clone());

    // Build router
    let app = create_router(app_state);

    // Start server with SO_REUSEADDR for faster restarts during development
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // Create socket with SO_REUSEADDR to allow immediate port reuse
    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;

    // Only enable SO_REUSEPORT in production to avoid duplicate processes during
    // development Note: set_reuse_port is only available on some platforms
    #[cfg(all(unix, not(target_os = "macos")))]
    if config.environment != "development" {
        socket.set_reuse_port(true)?;
    }

    socket.set_nonblocking(true)?;
    socket.bind(&addr.into())?;
    socket.listen(1024)?;

    // Convert socket2 socket to tokio listener
    let listener = tokio::net::TcpListener::from_std(socket.into())?;
    let local_addr = listener.local_addr()?;

    // Print to stdout so it always shows (even if RUST_LOG isn't set)
    println!("\n🚀 V1 API server is running!");
    println!("   → Local:   http://localhost:{}", local_addr.port());
    println!("   → Network: http://0.0.0.0:{}", local_addr.port());
    println!(
        "   → Docs:    http://localhost:{}/api/docs",
        local_addr.port()
    );
    println!(
        "   → Health:  http://localhost:{}/health\n",
        local_addr.port()
    );

    info!("V1 API server started on port {}", local_addr.port());

    axum::serve(listener, app).await?;

    Ok(())
}
