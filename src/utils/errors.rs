use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            ApiError::Redis(err) => {
                tracing::error!("Redis error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Cache error")
            }
            ApiError::Auth(ref err) => {
                tracing::warn!("Authentication error: {}", err);
                (StatusCode::UNAUTHORIZED, err.as_str())
            }
            ApiError::Validation(ref err) => {
                tracing::warn!("Validation error: {}", err);
                (StatusCode::BAD_REQUEST, err.as_str())
            }
            ApiError::NotFound(ref err) => {
                tracing::warn!("Not found: {}", err);
                (StatusCode::NOT_FOUND, err.as_str())
            }
            ApiError::BadRequest(ref err) => {
                tracing::warn!("Bad request: {}", err);
                (StatusCode::BAD_REQUEST, err.as_str())
            }
            ApiError::Internal(err) => {
                tracing::error!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            ApiError::Jwt(err) => {
                tracing::warn!("JWT error: {}", err);
                (StatusCode::UNAUTHORIZED, "Invalid token")
            }
            ApiError::Io(err) => {
                tracing::error!("IO error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "File operation error")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
