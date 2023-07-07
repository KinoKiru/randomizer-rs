use std::fmt;

// use std::fmt::{self};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

// Explicit implementation for printing [to_string()]
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Longitude: {} Latitude: {}",
            self.longitude, self.latitude
        )
    }
}
