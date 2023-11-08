use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPhone(String);

#[derive(Error, Debug)]
pub enum UserPhoneError {
    #[error("invalid length")]
    InvalidLength,
}

const PHONE_LENGTH: (usize, usize) = (11, 10);

impl UserPhone {
    pub fn new(phone: &str) -> Result<UserPhone> {
        if phone.len() == PHONE_LENGTH.0 || phone.len() == PHONE_LENGTH.1 {
            Ok(UserPhone(phone.to_string()))
        } else {
            Err(UserPhoneError::InvalidLength.into())
        }
    }
}

impl std::fmt::Display for UserPhone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserPhone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[test]
fn test_user_phone() {
    let phone = UserPhone::new("08012345678").unwrap();
    assert_eq!(phone.to_string(), "08012345678");
    let phone = UserPhone::new("0312345678").unwrap();
    assert_eq!(phone.to_string(), "0312345678");
    let phone = UserPhone::new("080123456789");
    assert_eq!(phone.is_err(), true);
    let phone = UserPhone::new("031234567");
    assert_eq!(phone.is_err(), true);
}
