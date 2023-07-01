use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RandomIntQuery {
    pub allow_negative: Option<bool>,
}
