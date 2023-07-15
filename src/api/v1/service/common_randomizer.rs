use std::fs;
use std::path::Path;
use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use prost_types::Timestamp;
use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

use crate::proto::common_service_server::CommonService;
use crate::proto::{
    CardRequest, CardResponse, DateResponse, Location, Season, SeasonResponse, TimeResponse,
};

#[derive(Debug, Default)]
pub struct RandomCommonController {}

impl Season {
    pub fn values() -> [Season; 4] {
        [
            Season::Spring,
            Season::Summer,
            Season::Autumn,
            Season::Winter,
        ]
    }
}

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
        // Create random thread on cpu
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let time = Duration::from_secs(rng.gen_range(0..(3600 * 24)));

        Ok(Response::new(TimeResponse {
            nanos: time.subsec_nanos(),
            secs: time.as_secs(),
        }))
    }

    async fn get_random_season(&self, _: Request<()>) -> Result<Response<SeasonResponse>, Status> {
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let seasons = Season::values();

        Ok(Response::new(SeasonResponse {
            season: seasons[rng.gen_range(0..seasons.len())].into(),
        }))
    }

    async fn get_random_location(&self, _: Request<()>) -> Result<Response<Location>, Status> {
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        Ok(Response::new(Location {
            longitude: rng.gen_range(-180.0..180.0),
            latitude: rng.gen_range(-90.0..90.0),
        }))
    }

    async fn get_random_date(&self, _: Request<()>) -> Result<Response<DateResponse>, Status> {
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        let date =
            NaiveDateTime::from_timestamp_opt(rng.gen_range(-946771200..Utc::now().timestamp()), 0)
                .ok_or(Status::internal(
                    "UTC Error highly likely to be out of bounds",
                ))?;
        let timestamp = Timestamp {
            seconds: date.timestamp_millis() / 1000,
            nanos: date.timestamp_nanos() as i32,
        };
        Ok(Response::new(DateResponse {
            date: Some(timestamp),
        }))
    }
}
