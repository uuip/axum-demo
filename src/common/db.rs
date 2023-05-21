use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn connection() -> AppState {
    let dsn = dotenvy::var("DATABASE_URL").unwrap();
    let conn = Database::connect(dsn).await.unwrap();
    AppState { conn }
}
