use actix_web::{get, web, Responder, Result};
use rand::{thread_rng, Rng};

use crate::api::data::string::{RandomFirstNameQuery, RandomPasswordQuery, RandomTextQuery};

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

    // Very cool loop
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

    // One way to do a loop
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
        // Either one is possible although include_str handles errors better
        // fs::read_to_string(Path::new("resource/text/loremipsum.txt")).expect("cannot read file");
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

#[get("/randomFirstName")]
async fn random_first_name(
    info: actix_web_validator::Query<RandomFirstNameQuery>,
) -> Result<impl Responder> {
    if !info.allow_boy_names && !info.allow_girl_names {
        return Err(actix_web::error::ErrorBadRequest(
            "Kan geen namen genereren als alles uitgeschakeld is",
        ));
    };

    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let mut names: Vec<String> = vec![];
    if info.allow_boy_names {
        names.append(
            &mut include_str!("..\\..\\..\\resource\\text\\jongensvoornamen.txt")
                .split_whitespace() // Split on any whitespace & turn into iter<str>
                .map(|e| e.to_string()) // Turn any item to String instead of str
                .collect::<Vec<String>>(),
        );
    };

    if info.allow_girl_names {
        names.append(
            &mut include_str!("..\\..\\..\\resource\\text\\meisjesvoornamen.txt")
                .split_whitespace()
                .map(|e| e.to_string())
                .collect::<Vec<String>>(),
        );
    };

    // Collect the random names to return to user
    let final_names: Vec<String> = (0..info.amount_of_names)
        .map(|_| names[rng.gen_range(0..names.len())].clone())
        .collect();

    Ok(web::Json(final_names))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/string")
        .service(random_password)
        .service(random_color)
        .service(random_text)
        .service(random_first_name)
}
