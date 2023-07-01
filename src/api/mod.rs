use actix_web::web;

pub mod data;
pub mod route;

pub fn routes() -> actix_web::Scope {
    web::scope("/api").service(route::number_randomizer::routes())
}
