use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};

use super::jwt;
use crate::common::ApiError;

#[derive(Deserialize)]
pub struct LoginInput {
    id: i64,
}

pub async fn login(Json(payload): Json<LoginInput>) -> Result<Json<Value>, ApiError> {
    let token = jwt::sign(payload.id)?;

    Ok(Json(json!({ "token": token })))
}
