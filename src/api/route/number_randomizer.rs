use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::number::{RandomDiceQuery, RandomIntQuery};

#[get("/randomInt")]
async fn random_int(info: web::Query<RandomIntQuery>) -> Result<impl Responder> {
    // Get query param allow negative, if not given default to true
    let allow_nullable: bool = info.allow_negative.unwrap_or(true);
    // Create random thread on cpu
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    // Match the value
    let random: i32 = match allow_nullable {
        true => rng.gen_range(i32::MIN..i32::MAX),
        false => rng.gen_range(0..i32::MAX),
    };

    Ok(web::Json(random))
}

#[get("/randomDice")]
async fn random_dice(info: actix_web_validator::Query<RandomDiceQuery>) -> Result<impl Responder> {
    // Get array size from query, if empty set default to 1
    let array_size: i32 = info.amount_of_dice.unwrap_or(1);
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    // Because we don't know how big the given array should be on compiling
    // We use a vec this will lengthen the array when command is called
    // This is thrown on the HEAP so it's less performant than a array which is on the STACK
    let mut vector: Vec<u8> = vec![];

    for _ in 0..array_size {
        vector.push(rng.gen_range(0..7))
    }

    Ok(web::Json(vector))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/number")
        .service(random_int)
        .service(random_dice)
}
