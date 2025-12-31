mod config;
mod proxy;
mod url_rewriter;

use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use config::Config;
use proxy::AppState;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nix_cache_proxy=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Loaded configuration: {:?}", config);

    // Create shared state
    let state = AppState {
        config: Arc::new(config.clone()),
        client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?,
    };

    // Build router
    let app = Router::new()
        .route("/{*path}", get(handle_request))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    tracing::info!("Nix cache proxy listening on {}", config.bind_address);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_request(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> impl axum::response::IntoResponse {
    tracing::info!("Received request for: {}", path);
    proxy::proxy_handler(State(state), path).await
}
