use anyhow::Result;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::str::FromStr;
use tokio_postgres::NoTls;

pub async fn connection() -> Result<Pool> {
    let db_url = dotenvy::var("DB_URL").expect("lost DB_URL");
    let mut pg_config = tokio_postgres::Config::from_str(&db_url)?;
    pg_config.options("-c LC_MESSAGES=en_US.UTF-8");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::builder(mgr).max_size(100).build().map_err(Into::into)
}
