use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/",
            web::get().to(|| async { HttpResponse::Ok().body("Hello World!") }),
        )
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
