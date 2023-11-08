use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(pub String);

#[derive(Error, Debug)]
pub enum UserIdError {
    #[error("uid is empty")]
    Empty,
    #[error("invalid length")]
    InvalidLength,
}

impl UserId {
    pub fn new(uid: &str) -> Result<UserId> {
        if uid.is_empty() {
            Err(UserIdError::Empty.into())
        } else if uid.len() != 28 {
            Err(UserIdError::InvalidLength.into())
        } else {
            Ok(UserId(uid.to_string()))
        }
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}
