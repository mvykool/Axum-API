use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Spotify API error: {0}")]
    SpotifyApi(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl IntoResponse for AppError {
    fn into_resopnse(self) -> Response {
        let (status, error_message) = match self {
            AppError::SpotifyApi(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::Request(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Config(msg) => (StatusCode::INTERVAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
