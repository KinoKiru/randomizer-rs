use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RandomCardQuery {
    pub allow_joker: bool,
}

impl Default for RandomCardQuery {
    fn default() -> RandomCardQuery {
        RandomCardQuery { allow_joker: true }
    }
}
