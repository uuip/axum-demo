use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub conn: PgPool,
}

pub async fn connection() -> PgPool {
    let dsn = dotenvy::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(4)
        .connect(&dsn)
        .await
        .unwrap()
}
