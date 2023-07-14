use async_std::fs;
use async_std::path::Path;
use async_std::stream::StreamExt;
use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

use crate::proto::common_service_server::CommonService;
use crate::proto::{CardRequest, CardResponse};

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
            .await
            .map_err(|e| Status::internal(e.to_string()));
        let files = Vec::new();

        while let Ok(child) = dir.next().await? {
            if child.metadata().await?.is_file() {
                files.push(child);
            } else {
                info!("random card while got into a dir");
            }
        }
        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let mut card = &files[rng.gen_range(0..files.len())];

        // If joker card is not allowed loop until other is found
        if !req.allow_joker {
            // While file name contains joker get new card
            while matches!(
                card.file_name().to_str(),
                Some(file_name) if file_name.contains("joker"))
            {
                rng = thread_rng();
                card = &files[rng.gen_range(0..files.len())]
            }
        }

        let image = fs::read(card.path()).await?;

        Ok(Response::new(CardResponse { card: image }))
    }
}
