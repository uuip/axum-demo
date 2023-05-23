use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub conn: PgPool,
}

pub async fn connection() -> PgPool {
    let dsn = dotenvy::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(3000)
        .min_connections(64)
        .connect(&dsn)
        .await
        .unwrap()
}
