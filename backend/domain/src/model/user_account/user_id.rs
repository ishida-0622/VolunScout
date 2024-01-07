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

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod test_user_id {
    use super::*;

    #[test]
    fn ok() {
        let uid = UserId::new("abcdefghijklmnopqrstuvwxyz12").unwrap();
        assert_eq!(uid.to_string(), "abcdefghijklmnopqrstuvwxyz12");
    }

    #[test]
    fn empty() {
        let uid = UserId::new("");
        assert_eq!(uid.is_err(), true);
    }

    #[test]
    fn too_long() {
        let uid = UserId::new("abcdefghijklmnopqrstuvwxyz123");
        assert_eq!(uid.is_err(), true);
    }

    #[test]
    fn too_short() {
        let uid = UserId::new("abcdefghijklmnopqrstuvwxyz1");
        assert_eq!(uid.is_err(), true);
    }
}
