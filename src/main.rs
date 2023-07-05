#[macro_use]
extern crate log;

use std::env;

use actix_web::{middleware, web, App, HttpServer};

mod api;
pub mod classes;
pub mod utils;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Enable rust backtrace for clearer errors
    std::env::set_var("RUST_BACKTRACE", "1");
    // Init log file
    log4rs::init_file("log4rs.yml", Default::default()).ok();

    // Init .env file
    dotenvy::dotenv().ok();

    // Get values from env
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("8080"));

    let conn = utils::database::initialize(&db_url).await
        .map_err(|e| error!("{:#?}", e))
        .expect("LOL database error");

    let server_url = format!("{host}:{port}");
    // Log server url
    info!("Running server on {}", server_url);

    // Add compression to api and add database connection to each request
    HttpServer::new(move || App::new()
    .wrap(middleware::Compress::default())
    .service(api::routes())
    .app_data(web::Data::new(conn.clone())))
        .bind(server_url)?
        .run()
        .await
}
