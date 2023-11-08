use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserName(String);

#[derive(Debug, Clone, Error)]
pub enum UserNameError {
    #[error("user name is empty")]
    Empty,
    #[error("user name is too long")]
    TooLong,
}

impl UserName {
    pub fn new(name: &str) -> Result<UserName> {
        if name.is_empty() {
            Err(UserNameError::Empty.into())
        } else if name.len() > 50 {
            Err(UserNameError::TooLong.into())
        } else {
            Ok(UserName(name.to_string()))
        }
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[test]
fn test_user_name() {
    let name = UserName::new("石田健太郎").unwrap();
    assert_eq!(name.to_string(), "石田健太郎");
}
