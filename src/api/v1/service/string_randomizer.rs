use rand::{thread_rng, Rng};
use tonic::{Request, Response, Status};

use crate::proto::string_service_server::{StringService, StringServiceServer};
use crate::proto::{
    ColorResponse, NameRequest, NameResponse, PasswordRequest, PasswordResponse, TextRequest,
    TextResponse,
};
use crate::utils::v1::custom_macro::rules;

#[derive(Debug, Default)]
pub struct RandomStringController {}

#[tonic::async_trait]
impl StringService for RandomStringController {
    async fn get_random_password(
        &self,
        request: Request<PasswordRequest>,
    ) -> Result<Response<PasswordResponse>, Status> {
        let req = request.get_ref();

        if !req.allow_lowercase && !req.allow_numbers && !req.allow_specials && !req.allow_uppercase
        {
            return Err(Status::invalid_argument(
                "Cannot make password if you set everything to false",
            ));
        };

        let mut possibles: String = "".to_string();
        if req.allow_uppercase {
            possibles += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        }

        if req.allow_lowercase {
            possibles += "abcdefghijklmnopqrstuvwxyz";
        }

        if req.allow_specials {
            possibles += "$%#@!*?;:^&";
        }

        if req.allow_numbers {
            possibles += "1234567890";
        }

        let possible_array: &Vec<char> = &possibles.chars().collect();
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        // Very cool loop
        let password: String = (0..req.length)
            .map(|_| possible_array[rng.gen_range(0..possibles.len())])
            .collect();

        Ok(Response::new(PasswordResponse { password: password }))
    }

    async fn get_random_color(
        &self,
        _: Request<()>,
    ) -> Result<tonic::Response<ColorResponse>, Status> {
        let mut color: String = "#".to_string();
        let possibles: Vec<char> = "ABCDEF0123456789".chars().collect();
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        // One way to do a loop
        for _ in 0..6 {
            color += &possibles[rng.gen_range(0..possibles.len())].to_string();
        }

        Ok(Response::new(ColorResponse { color: color }))
    }

    async fn get_random_text(
        &self,
        request: Request<TextRequest>,
    ) -> Result<tonic::Response<TextResponse>, Status> {
        let req = request.get_ref();
        let mut string: String = "".to_string();
        let mut rng: rand::rngs::ThreadRng = thread_rng();

        let full_text: String = if req.use_english {
            // Either one is possible although include_str handles errors better
            // fs::read_to_string(Path::new("resource/text/loremipsum.txt")).expect("cannot read file");
            include_str!("../../../../resource/text/loremipsum.txt").to_string()
        } else {
            include_str!("../../../../resource/text/loremipsumNL.txt").to_string()
        };
        let paragraph: Vec<&str> = full_text.split("--").collect::<Vec<&str>>();

        for _ in 0..req.amount_of_paragraphs {
            string += &if req.use_html {
                format!("<p>{}</p>", &paragraph[rng.gen_range(0..paragraph.len())])
            } else {
                paragraph[rng.gen_range(0..paragraph.len())].to_string()
            };
        }

        Ok(Response::new(TextResponse { text: string }))
    }

    async fn get_random_name(
        &self,
        request: Request<NameRequest>,
    ) -> Result<tonic::Response<NameResponse>, tonic::Status> {
        let req = request.get_ref();

        if !req.allow_boy_names && !req.allow_girl_names {
            return Err(Status::invalid_argument(
                "Cannot generate a name if both girl and boy names are set to off",
            ));
        };

        let mut rng: rand::rngs::ThreadRng = thread_rng();
        let mut names: Vec<String> = vec![];
        if req.allow_boy_names {
            names.append(
                &mut include_str!("../../../../resource/text/jongensvoornamen.txt")
                    .split_whitespace() // Split on any whitespace & turn into iter<str>
                    .map(|e| e.to_string()) // Turn any item to String instead of str
                    .collect::<Vec<String>>(),
            );
        };

        if req.allow_girl_names {
            names.append(
                &mut include_str!("../../../../resource/text/meisjesvoornamen.txt")
                    .split_whitespace()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>(),
            );
        };

        // Collect the random names to return to user
        let final_names: Vec<String> = (0..req.amount_of_names)
            .map(|_| names[rng.gen_range(0..names.len())].clone())
            .collect();

        Ok(Response::new(NameResponse { names: final_names }))
    }
}
rules::initialize_route!(StringServiceServer, RandomStringController);
