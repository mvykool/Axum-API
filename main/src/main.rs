mod config;
mod error;
mod handlers;
mod models;
mod services;

use axum::{Router, routing::get};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use config::Config;
use services::spotify::SpotifyService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("failed to load configuration");
    let port = config.port;

    let spotify_service = Arc::new(SpotifyService::new(config));

    let app = Router::new()
        .route("/api/spotify", get(handlers::spotify::now_playing))
        .route("/health", get(|| async { "OK" }))
        .layer(CorsLayer::permissive())
        .with_state(spotify_service);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    println!("Server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("failed to start server");
}
