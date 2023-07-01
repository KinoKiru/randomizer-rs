use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::number::RandomIntQuery;

#[get("/randomInt")]
async fn get_random_int(info: web::Query<RandomIntQuery>) -> Result<impl Responder> {
    let allow_nullable = info.allow_negative;
    let mut rng = thread_rng();
    let random = match allow_nullable {
        Some(true) => rng.gen_range(i32::MIN..i32::MAX),
        Some(false) => rng.gen_range(0..i32::MAX),
        None => {
            println!("{:?}", allow_nullable);
            rng.gen_range(0..i32::MAX)
        }
    };

    Ok(web::Json(random))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/number").service(get_random_int)
}
