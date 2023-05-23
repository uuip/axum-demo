use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::common::ApiError;

const JWT_SECRET: &str = "aaabbb";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i64,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: i64) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::days(90);

        Self {
            id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| ApiError::MissingCredentials)?;
        verify(bearer.token())
    }
}

pub fn sign(id: i64) -> Result<String, ApiError> {
    jsonwebtoken::encode(
        &Header::default(),
        &Claims::new(id),
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(ApiError::JwtError)
}

pub fn verify(token: &str) -> Result<Claims, ApiError> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(ApiError::JwtError)
}
