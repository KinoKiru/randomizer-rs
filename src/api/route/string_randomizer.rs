use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::string::RandomPasswordQuery;

// TODO error handeling?
#[get("/randomPassword")]
async fn random_password(
    info: actix_web_validator::Query<RandomPasswordQuery>,
) -> Result<impl Responder> {
    let mut password: String = "".to_string();
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    if !info.allow_lowercase && !info.allow_numbers && !info.allow_specials && !info.allow_uppercase
    {
        return Ok(web::Json(
            "Cannot make password if you set everything to false".to_string(),
        ));
    };

    let mut possibles: String = "".to_string();
    if info.allow_uppercase {
        possibles += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    if info.allow_lowercase {
        possibles += "abcdefghijklmnopqrstuvwxyz";
    }

    if info.allow_specials {
        possibles += "$%#@!*?;:^&";
    }

    if info.allow_numbers {
        possibles += "1234567890";
    }

    let possible_array: &Vec<char> = &possibles.chars().collect::<Vec<char>>();
    for _ in 0..info.length {
        password += &possible_array[rng.gen_range(0..possibles.len())].to_string();
    }

    Ok(web::Json(password))
}

#[get("/randomColor")]
async fn random_color() -> Result<impl Responder> {
    let mut color: String = "#".to_string();
    let possibles: &str = "ABCDEF0123456789";
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    for _ in 0..6 {
        color += &possibles.chars().collect::<Vec<char>>()[rng.gen_range(0..possibles.len())]
            .to_string();
    }

    Ok(web::Json(color))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/string")
        .service(random_password)
        .service(random_color)
}
