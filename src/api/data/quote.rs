use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuoteQuery {
    #[validate(length(min = 1))]
    pub text: String,
}
