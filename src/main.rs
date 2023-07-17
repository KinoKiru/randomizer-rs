#[macro_use]
extern crate log;

use std::env;

use tonic::service::interceptor;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

pub mod api;
pub mod utils;

pub mod proto {
    tonic::include_proto!("randomizer.v1");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("descriptor");
}

#[rustfmt::skip]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let conn = utils::v1::database::initialize(&db_url).await
        .map_err(|e| error!("{:#?}", e))
        .expect("Database error");

    let server_url = format!("{host}:{port}");
    // Log server url
    info!("Running server on {}", server_url);


    Server::builder()
    .layer(interceptor(move |mut req: tonic::Request<()>| {
        req.extensions_mut().insert(conn.clone());
        Ok(req)
    }))
    .add_service(api::v1::service::common_randomizer::server())
    .add_service(Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?
    )
    .serve(server_url.parse().expect("lmao string could not be parsed"))
    .await?;

Ok(())
}
