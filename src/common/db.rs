use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AppState {
    pub conn: Pool,
}

pub async fn connection() -> Pool {
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    let dsn = url::Url::parse(&db_url).unwrap();
    let mut cfg = Config::new();
    cfg.host = dsn.host().map(|x| x.to_string());
    cfg.port = dsn.port();
    cfg.dbname = dsn.path_segments().unwrap().next().map(|x| x.to_string());
    cfg.user = Some(dsn.username().to_string());
    cfg.password = dsn.password().map(|x| x.to_string());
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
