use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
#[serde(default)]
pub struct RandomPasswordQuery {
    #[validate(range(min = 1, max = 250))]
    pub length: i32,
    pub allow_lowercase: bool,
    pub allow_uppercase: bool,
    pub allow_specials: bool,
    pub allow_numbers: bool,
}

impl Default for RandomPasswordQuery {
    fn default() -> RandomPasswordQuery {
        RandomPasswordQuery {
            length: 10,
            allow_uppercase: true,
            allow_lowercase: true,
            allow_numbers: true,
            allow_specials: true,
        }
    }
}

// pub trait ValidateArgs<'v_a> {
//     type Args;

//     fn validate_args(&self, args: Self::Args) -> Result<(), ValidationErrors>;
// }

// #[validate(custom(
//     function = "validate_options",
//     arg = "(&'v_a Option(bool), &'v_a Option(bool), &'v_a Option(bool))"
// ))]
// pub allow_lowercase: Option<bool>,

// Imagine making your own validation
// pub fn validate_options(
//     allow_lowercase: &bool,
//     arg: (Option<bool>, Option<bool>, Option<bool>),
// ) -> Result<(), ValidationError> {
//     if !allow_lowercase
//         && !arg.0.unwrap_or_default()
//         && !arg.1.unwrap_or_default()
//         && !arg.2.unwrap_or_default()
//     {
//         return Err(ValidationError::new(
//             "Cannot make password if you set everything to false",
//         ));
//     }
//     Ok(())
// }
