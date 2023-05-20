use axum::{http::StatusCode, Json};
use sea_orm::DbErr;
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error("not found")]
    NotFound,
    #[error("failed to create user")]
    FailedCreateUser,
}

pub type ApiError = (StatusCode, Json<Value>);

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        let status = match err {
            Error::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let payload = json!({"message": err.to_string()});
        (status, Json(payload))
    }
}
