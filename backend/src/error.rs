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
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error occurred")
            }
            ApiError::Auth(ref msg) => (StatusCode::UNAUTHORIZED, msg.as_str()),
            ApiError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::NotFound(ref msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            ApiError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            ApiError::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
