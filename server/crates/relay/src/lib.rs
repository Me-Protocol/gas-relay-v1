//! This module contains the relayer axum server

use axum::{
    http::Method,
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use primitives::configs::ServerConfig;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
pub mod error;

pub struct AppState {
    pub db_url: String,
}

/// Run the relayer server
pub async fn run_relayer_server(config: ServerConfig) -> Result<(), anyhow::Error> {
    let url = config.server_url.clone();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    let app_state = Arc::new(AppState {
        db_url: config.db_url.clone(),
    });

    let app = Router::new()
        .route("/", get(|| async { "Gasless Relayer." }))
        .layer(cors)
        .with_state(app_state);

    tracing::info!(url);
    axum::serve(TcpListener::bind(url).await.unwrap(), app)
        .await
        .unwrap();

    Ok(())
}
