use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male = 0,
    Female = 1,
    Other = 2,
}

#[derive(Error, Debug)]
pub enum GenderError {
    #[error("gender not found")]
    NotFound,
}

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

pub fn gender_from_i8(arg: &i8) -> Result<Gender> {
    match arg {
        0 => Ok(Gender::Male),
        1 => Ok(Gender::Female),
        2 => Ok(Gender::Other),
        _ => Err(GenderError::NotFound.into()),
    }
}
