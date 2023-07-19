use entity::quote;
use migration::sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};
use migration::Expr;
use tonic::{Request, Response, Status};

use crate::proto::quote_service_server::{QuoteService, QuoteServiceServer};
use crate::proto::{CreateQuoteRequest, Quote};
use crate::utils::v1::custom_macro::rules;

#[derive(Debug, Default)]
pub struct RandomQuoteController {}

#[tonic::async_trait]
impl QuoteService for RandomQuoteController {
    async fn get_random_quotes(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<Quote>, Status> {
        let db = request.extensions().get::<DatabaseConnection>().unwrap();

        let item: Option<quote::Model> = quote::Entity::find()
            .limit(1)
            .order_by(Expr::cust("RANDOM()"), migration::Order::Asc)
            .one(db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(item) = item {
            Ok(Response::new(Quote {
                id: item.id,
                text: item.text,
            }))
        } else {
            Err(Status::internal("No quote found, maybe make one?"))
        }
    }

    async fn create_quote(
        &self,
        request: Request<CreateQuoteRequest>,
    ) -> Result<tonic::Response<Quote>, Status> {
        let db = request.extensions().get::<DatabaseConnection>().unwrap();
        let req = request.get_ref();

        let new_quote = quote::ActiveModel {
            text: migration::sea_orm::ActiveValue::Set(req.quote.as_ref().unwrap().text.clone()),
            ..Default::default()
        };

        let new_quote: quote::Model = new_quote
            .insert(db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Quote {
            id: new_quote.id,
            text: new_quote.text,
        }))
    }
}

rules::initialize_route!(QuoteServiceServer, RandomQuoteController);
