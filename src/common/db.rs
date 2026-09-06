use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn connection() -> Result<DatabaseConnection> {
    let dsn = dotenvy::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(dsn);
    opt.max_connections(100).min_connections(4);
    Database::connect(opt).await.map_err(Into::into)
}
