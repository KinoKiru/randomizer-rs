use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize)]
pub struct RandomIntQuery {
    pub allow_negative: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RandomDiceQuery {
    #[validate(range(min = 0, max = 10000))]
    pub amount_of_dice: Option<i32>,
}
