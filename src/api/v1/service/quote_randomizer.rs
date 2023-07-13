use actix_web::error::ErrorInternalServerError;
use actix_web::{get, post, web, Responder, Result};
use entity::quote;
use migration::sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};
use migration::Expr;

use crate::api::data::quote::CreateQuoteQuery;

#[get("/randomQuote")]
async fn get_random_quote(conn: web::Data<DatabaseConnection>) -> Result<impl Responder> {
    let db = conn.as_ref();

    let item: Option<quote::Model> = quote::Entity::find()
        .limit(1)
        .order_by(Expr::cust("RANDOM()"), migration::Order::Asc)
        .one(db)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    if let Some(item) = item {
        Ok(web::Json(item.text))
    } else {
        Err(actix_web::error::ErrorNotFound(
            "No quote found, maybe make one?",
        ))
    }
}

#[post("/randomQuote")]
async fn store_random_quote(
    conn: web::Data<DatabaseConnection>,
    info: actix_web_validator::Query<CreateQuoteQuery>,
) -> Result<impl Responder> {
    let db = conn.as_ref();

    let new_quote = quote::ActiveModel {
        text: migration::sea_orm::ActiveValue::Set(info.text.clone()),
        ..Default::default()
    };

    let new_quote: quote::Model = new_quote
        .insert(db)
        .await
        .map_err(|e| ErrorInternalServerError(e))?;

    Ok(web::Json(new_quote.text))
}

pub fn routes() -> actix_web::Scope {
    web::scope("/string")
        .service(get_random_quote)
        .service(store_random_quote)
}
