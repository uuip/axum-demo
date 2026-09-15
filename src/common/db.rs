use anyhow::{Context, Result};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;

pub fn connection() -> Result<Pool> {
    let db_url = dotenvy::var("DB_URL").context("failed to read DB_URL")?;
    let mut pg_config = db_url
        .parse::<tokio_postgres::Config>()
        .context("invalid DB_URL")?;
    pg_config.options("-c LC_MESSAGES=en_US.UTF-8");
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(pg_config, NoTls, manager_config);
    Pool::builder(manager)
        .max_size(100)
        .build()
        .map_err(Into::into)
}
