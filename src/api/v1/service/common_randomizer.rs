use std::fs;
use std::path::Path;

use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

use crate::proto::common_service_server::CommonService;
use crate::proto::{
    CardRequest, CardResponse, DateResponse, Location, SeasonResponse, TimeResponse,
};

#[derive(Debug, Default)]
pub struct RandomCommonController {}

#[tonic::async_trait]
impl CommonService for RandomCommonController {
    async fn get_random_card(
        &self,
        req: Request<CardRequest>,
    ) -> Result<Response<CardResponse>, Status> {
        // Get values from request
        let req = req.get_ref();

        let dir = fs::read_dir(Path::new("resource/cards"))
            .map_err(|e| Status::internal(e.to_string()))?;
        let files = dir.collect::<Vec<_>>();

        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let mut card = files[rng.gen_range(0..files.len())].as_ref();

        // If joker card is not allowed loop until other is found
        if !req.allow_joker {
            // While file name contains joker get new card
            while matches!(
                card.expect("Card should exist").file_name().to_str(),
                Some(file_name) if file_name.contains("joker"))
            {
                rng = thread_rng();
                card = files[rng.gen_range(0..files.len())].as_ref()
            }
        }

        let image = fs::read(card.unwrap().path()).map_err(|e| Status::internal(e.to_string()))?;
        Ok(Response::new(CardResponse { card: image }))
    }

    async fn get_random_time(&self, _: Request<()>) -> Result<Response<TimeResponse>, Status> {
        todo!()
    }

    async fn get_random_season(&self, _: Request<()>) -> Result<Response<SeasonResponse>, Status> {
        todo!()
    }

    async fn get_random_location(&self, _: Request<()>) -> Result<Response<Location>, Status> {
        todo!()
    }

    async fn get_random_date(&self, _: Request<()>) -> Result<Response<DateResponse>, Status> {
        todo!()
    }
}
