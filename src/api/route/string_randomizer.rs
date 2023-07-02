use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::string::RandomPasswordQuery;

// TODO error handeling and remove unwraps ig?
#[get("/randomPassword")]
async fn random_password(
    info: actix_web_validator::Query<RandomPasswordQuery>,
) -> Result<impl Responder> {
    let mut password: String = "".to_string();
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    if !info.allow_lowercase.unwrap_or(true)
        && !info.allow_numbers.unwrap_or(true)
        && !info.allow_specials.unwrap_or(true)
        && !info.allow_uppercase.unwrap_or(true)
    {
        return Ok(web::Json(
            "Cannot make password if you set everything to false".to_string(),
        ));
    };

    let mut possibles: String = "".to_string();
    possibles += if info.allow_uppercase.unwrap_or(true) {
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    } else {
        ""
    };
    possibles += if info.allow_lowercase.unwrap_or(true) {
        "abcdefghijklmnopqrstuvwxyz"
    } else {
        ""
    };
    possibles += if info.allow_specials.unwrap_or(true) {
        "$%#@!*?;:^&"
    } else {
        ""
    };
    possibles += if info.allow_numbers.unwrap_or(true) {
        "1234567890"
    } else {
        ""
    };

    let possible_array = &possibles.chars().collect::<Vec<char>>();
    for _ in 0..info.length.unwrap_or(10) {
        password += &possible_array[rng.gen_range(0..possibles.len())].to_string();
    }

    Ok(web::Json(password))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/string").service(random_password)
}
