//! define error, and convert other Error type to ApiError without `imp From`

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

/// # ApiError
#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    SQLError(#[from] tokio_postgres::Error),
    #[error(transparent)]
    PoolError(#[from] deadpool_postgres::PoolError),
    #[error("not found")]
    NotFound,
    #[error("page params error")]
    PageError,
    #[error("failed to create user")]
    FailedCreateUser,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::WrongCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::BAD_REQUEST,
        };
        let body = Json(json!({
            "error": self.to_string(),
        }));
        (status, body).into_response()
    }
}

// impl From<Error> for ApiError
