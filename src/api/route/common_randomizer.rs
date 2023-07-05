use std::fs;
use std::path::Path;
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder, Result};
use chrono::{NaiveDateTime, Utc};
use rand::{thread_rng, Rng};

use crate::api::data::common::RandomCardQuery;
use crate::classes::location::Location;
use crate::classes::season::Season;

#[get("/randomTime")]
async fn random_time() -> Result<impl Responder> {
    // Create random thread on cpu
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    Ok(web::Json(Duration::from_secs(
        rng.gen_range(0..(3600 * 24)),
    )))
}

#[get("/randomSeason")]
async fn random_season() -> Result<impl Responder> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    // Enum goes crazy
    Ok(web::Json(Season::values()[rng.gen_range(0..4)].to_string()))
}

#[get("/randomLocation")]
async fn random_location() -> Result<impl Responder> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();

    let object = Location {
        longitude: rng.gen_range(-180.0..180.0),
        latitude: rng.gen_range(-90.0..90.0),
    };

    Ok(web::Json(object.to_string()))
}

#[get("/randomDate")]
async fn random_date() -> Result<impl Responder> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let dt =
        NaiveDateTime::from_timestamp_opt(rng.gen_range(-946771200..Utc::now().timestamp()), 0)
            .ok_or(actix_web::error::ErrorInternalServerError(
                "lmao error utc error",
            ))?;

    Ok(web::Json(dt.format("%Y-%m-%d %H:%M:%S").to_string()))
}

#[get("/randomCard")]
async fn random_card(info: web::Query<RandomCardQuery>) -> Result<impl Responder> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    let dir = fs::read_dir(Path::new("resource/cards")).unwrap();
    let files = dir.collect::<Vec<_>>();
    let mut card = files[rng.gen_range(0..files.len())].as_ref();

    if !info.allow_joker {
        while card
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .contains("joker")
        {
            card = files[rng.gen_range(0..files.len())].as_ref()
        }
    }

    fs::read(card.unwrap().path())
        .map(|file| {
            HttpResponse::build(StatusCode::OK)
                .content_type("image/png")
                .body(file)
        })
        .map_err(|_| actix_web::error::ErrorInternalServerError("cannot read file"))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/common")
        .service(random_time)
        .service(random_season)
        .service(random_location)
        .service(random_date)
        .service(random_card)
}
