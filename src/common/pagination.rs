use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use derive_builder::Builder;
use serde::Deserialize;
use validator::Validate;

use super::error::ApiError;

#[derive(Debug, Clone, Copy, Validate, Builder, Deserialize)]
pub struct Pagination {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 1))]
    pub size: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            size: Some(10),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let mut value =
            serde_urlencoded::from_str::<Self>(query).map_err(|_| ApiError::PageError)?;
        if value.size.is_none() {
            value.size = Some(10)
        }
        value.validate()?;
        Ok(value)
    }
}
