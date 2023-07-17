use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

use crate::proto::number_service_server::{NumberService, NumberServiceServer};
use crate::proto::{DiceRequest, DiceResponse, IntRequest, IntResponse};
use crate::utils::v1::custom_macro::rules;

#[derive(Debug, Default)]
pub struct RandomIntController {}

#[tonic::async_trait]
impl NumberService for RandomIntController {
    async fn get_random_dice(
        &self,
        req: Request<DiceRequest>,
    ) -> Result<Response<DiceResponse>, Status> {
        let req = req.get_ref();
        // Create random thread on cpu
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        // Because we don't know how big the given array should be on compiling
        // We use a vec this will lengthen the array when command is called
        // This is thrown on the HEAP so it's less performant than a array which is on the STACK
        let vector: Vec<i32> = (0..req.amount_of_dice)
            .map(|_| rng.gen_range(0..7))
            .collect();

        Ok(Response::new(DiceResponse { dices: vector }))
    }

    async fn get_random_int(
        &self,
        req: Request<IntRequest>,
    ) -> Result<Response<IntResponse>, Status> {
        let req = req.get_ref();
        // Create random thread on cpu
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        // Match the value
        let random: i32 = match req.allow_negative {
            true => rng.gen_range(i32::MIN..i32::MAX),
            false => rng.gen_range(0..i32::MAX),
        };

        Ok(Response::new(IntResponse { number: random }))
    }
}
rules::initialize_route!(NumberServiceServer, RandomIntController);
