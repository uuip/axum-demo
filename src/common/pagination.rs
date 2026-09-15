use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use serde::Deserialize;
use validator::Validate;

use super::error::ApiError;

#[derive(Debug, Clone, Copy, Validate, Deserialize)]
pub struct Pagination {
    #[validate(range(min = 1))]
    pub page: u64,
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1))]
    pub size: u64,
}

fn default_page_size() -> u64 {
    10
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            size: default_page_size(),
        }
    }
}

impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_urlencoded::from_str::<Self>(query).map_err(|_| ApiError::PageError)?;
        value.validate()?;
        Ok(value)
    }
}
