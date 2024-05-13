use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connection() -> PgPool {
    let dsn = dotenvy::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(100)
        .min_connections(4)
        .connect(&dsn)
        .await
        .unwrap()
}
