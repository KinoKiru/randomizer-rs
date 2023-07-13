use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::number::{RandomDiceQuery, RandomIntQuery};

#[get("/randomInt")]
async fn random_int(info: web::Query<RandomIntQuery>) -> Result<impl Responder> {
    // Create random thread on cpu
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    // Match the value
    let random: i32 = match info.allow_negative {
        true => rng.gen_range(i32::MIN..i32::MAX),
        false => rng.gen_range(0..i32::MAX),
    };

    Ok(web::Json(random))
}

#[get("/randomDice")]
async fn random_dice(info: actix_web_validator::Query<RandomDiceQuery>) -> Result<impl Responder> {
    // Create random thread on cpu
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    // Because we don't know how big the given array should be on compiling
    // We use a vec this will lengthen the array when command is called
    // This is thrown on the HEAP so it's less performant than a array which is on the STACK
    let vector: Vec<i32> = (0..info.amount_of_dice)
        .map(|_| rng.gen_range(0..7))
        .collect();

    Ok(web::Json(vector))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/number")
        .service(random_int)
        .service(random_dice)
}
