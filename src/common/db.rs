use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn connection() -> AppState {
    let dsn = dotenvy::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(dsn.to_string());
    opt.max_connections(1000);
    // let conn = Database::connect(dsn).await.unwrap();
    let conn = Database::connect(opt).await.unwrap();
    AppState { conn }
}
