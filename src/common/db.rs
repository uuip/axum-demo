use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connection() -> Result<PgPool> {
    let database_url = dotenvy::var("DATABASE_URL").context("failed to read DATABASE_URL")?;
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(4)
        .connect(&database_url)
        .await
        .map_err(Into::into)
}
