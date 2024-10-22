use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub async fn init_pool() -> Result<Pool, Box<dyn std::error::Error>> {
    dotenv().ok();

    // Set up PostgreSQL config
    let mut cfg = Config::new();
    cfg.dbname = Some(env::var("DB_NAME").unwrap_or_else(|_| "my_database".to_string()));
    cfg.user = Some(env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()));
    cfg.password = Some(env::var("DB_PASSWORD").unwrap_or_else(|_| "mysecretpassword".to_string()));
    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));

    // Create a connection pool
    let pool = cfg.create_pool(None, NoTls)?;

    Ok(pool)
}
