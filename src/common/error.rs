use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
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

// pub type ApiError = (StatusCode, Json<Value>);
//
// impl From<Error> for ApiError {
//     fn from(err: Error) -> Self {
//         let status = match err {
//             Error::NotFound => StatusCode::NOT_FOUND,
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         };
//         let payload = json!({"message": err.to_string()});
//         (status, Json(payload))
//     }
// }
