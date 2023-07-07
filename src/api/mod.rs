use actix_web::web;

pub mod data;
pub mod route;

// Set routes with a scope
pub fn routes() -> actix_web::Scope {
    web::scope("/api")
        .service(route::number_randomizer::routes())
        .service(route::string_randomizer::routes())
        .service(route::common_randomizer::routes())
        .service(route::quote_randomizer::routes())
}
