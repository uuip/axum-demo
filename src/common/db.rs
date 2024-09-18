use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn connection() -> Result<AppState> {
    let dsn = dotenvy::var("DATABASE_URL")?;
    let mut opt = ConnectOptions::new(dsn.to_string());
    opt.max_connections(100).min_connections(4);
    let conn = Database::connect(opt).await?;
    Ok(AppState { conn })
}
