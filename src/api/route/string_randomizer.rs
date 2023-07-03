use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::string::{RandomPasswordQuery, RandomTextQuery};

#[get("/randomPassword")]
async fn random_password(
    info: actix_web_validator::Query<RandomPasswordQuery>,
) -> Result<impl Responder> {
    if !info.allow_lowercase && !info.allow_numbers && !info.allow_specials && !info.allow_uppercase
    {
        return Err(actix_web::error::ErrorBadRequest(
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

    let possible_array: &Vec<char> = &possibles.chars().collect();
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    let password: String = (0..info.length)
        .map(|_| possible_array[rng.gen_range(0..possibles.len())])
        .collect();

    Ok(web::Json(password))
}

#[get("/randomColor")]
async fn random_color() -> Result<impl Responder> {
    let mut color: String = "#".to_string();
    let possibles: Vec<char> = "ABCDEF0123456789".chars().collect();
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    for _ in 0..6 {
        color += &possibles[rng.gen_range(0..possibles.len())].to_string();
    }

    Ok(web::Json(color))
}

#[get("/randomText")]
async fn random_text(info: actix_web_validator::Query<RandomTextQuery>) -> Result<impl Responder> {
    let mut string: String = "".to_string();
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    let full_text: String = if info.use_english {
        include_str!("..\\..\\..\\resource\\text\\loremipsum.txt").to_string()
    } else {
        include_str!("..\\..\\..\\resource\\text\\loremipsumNL.txt").to_string()
    };
    let paragraph: Vec<&str> = full_text.split("--").collect::<Vec<&str>>();

    for _ in 0..info.amount_of_paragraphs {
        string += &if info.use_html {
            format!("<p>{}</p>", &paragraph[rng.gen_range(0..paragraph.len())])
        } else {
            paragraph[rng.gen_range(0..paragraph.len())].to_string()
        };
    }

    match info.use_html {
        true => Ok(string),
        false => Ok(format!(r#""{}""#, string)),
    }
}

pub fn routes() -> actix_web::Scope {
    web::scope("/string")
        .service(random_password)
        .service(random_color)
        .service(random_text)
}
