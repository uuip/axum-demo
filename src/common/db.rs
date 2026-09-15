use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connection() -> Result<DatabaseConnection> {
    let database_url = dotenvy::var("DATABASE_URL").context("failed to read DATABASE_URL")?;
    let mut options = ConnectOptions::new(database_url);
    options.max_connections(100).min_connections(4);
    Database::connect(options).await.map_err(Into::into)
}
