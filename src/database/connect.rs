use crate::config::load_config;
use sqlx::{PgPool};

pub async fn connect() -> PgPool {
    let config = load_config();
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.pg_user,
        config.pg_password,
        config.pg_host,
        config.pg_port,
        config.pg_db);
    PgPool::connect(&db_url).await.expect("failed to connect to database")
}