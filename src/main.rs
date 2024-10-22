mod db;
mod routes;
mod models;
mod auth;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use deadpool_postgres::Pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Setup logging
    env_logger::init();

    // Initialize PostgreSQL pool
    let pool: Pool = db::init_pool().await.expect("Failed to create pool");

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // Pass pool to routes
            .configure(routes::config_routes)       // Set up routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
