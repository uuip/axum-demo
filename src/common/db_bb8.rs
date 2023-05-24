use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use bb8_postgres::bb8::Pool as bb8Pool;

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool,
}
pub type Pool = bb8Pool<PostgresConnectionManager<NoTls>>;

pub async fn connection() -> Pool {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let manager=PostgresConnectionManager::new_from_stringlike(&db_url,NoTls).unwrap();
    Pool::builder().build(manager).await.unwrap()
}
