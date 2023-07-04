use actix_web::{App, HttpServer};
mod api;
pub mod classes;
pub mod utils;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(api::routes()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
