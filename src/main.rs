use actix_web::{web, App, HttpServer};
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Set up Postgres config
    let mut cfg = Config::new();
    cfg.dbname = Some(env::var("DB_NAME").expect("DB_NAME must be set"));
    cfg.user = Some(env::var("DB_USER").expect("DB_USER must be set"));
    cfg.password = Some(env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"));
    cfg.host = Some(env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()));

    // Create connection pool
    let pool: Pool = cfg.create_pool(NoTls).unwrap();

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
