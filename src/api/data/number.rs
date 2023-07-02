use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RandomIntQuery {
    pub allow_negative: bool,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(default)]
pub struct RandomDiceQuery {
    #[validate(range(min = 0, max = 10000))]
    pub amount_of_dice: i32,
}

impl Default for RandomDiceQuery {
    fn default() -> RandomDiceQuery {
        RandomDiceQuery { amount_of_dice: 10 }
    }
}

impl Default for RandomIntQuery {
    fn default() -> RandomIntQuery {
        RandomIntQuery {
            allow_negative: true,
        }
    }
}
