use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Other = 2,
}

#[derive(Debug)]
pub struct GenderNotFoundError;

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gender = match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Other => "Other",
        };
        write!(f, "{}", gender)
    }
}

pub fn gender_from_i8(arg: &i8) -> Result<Gender, GenderNotFoundError> {
    match arg {
        0 => Ok(Gender::Male),
        1 => Ok(Gender::Female),
        2 => Ok(Gender::Other),
        _ => Err(GenderNotFoundError),
    }
}
