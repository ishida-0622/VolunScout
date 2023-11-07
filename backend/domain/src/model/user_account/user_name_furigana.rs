use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNameFurigana(String);

#[derive(Debug, Clone, Error)]
pub enum UserNameFuriganaError {
    #[error("user name furigana is empty")]
    Empty,
    #[error("user name furigana is too long")]
    TooLong,
}

impl UserNameFurigana {
    pub fn new(furigana: &str) -> Result<UserNameFurigana, UserNameFuriganaError> {
        if furigana.is_empty() {
            Err(UserNameFuriganaError::Empty.into())
        } else if furigana.len() > 50 {
            Err(UserNameFuriganaError::TooLong.into())
        } else {
            Ok(UserNameFurigana(furigana.to_string()))
        }
    }
}

impl std::fmt::Display for UserNameFurigana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_user_furigana() {
    let furigana = UserNameFurigana::new("イシダケンタロウ").unwrap();
    assert_eq!(furigana.to_string(), "イシダケンタロウ");
}
