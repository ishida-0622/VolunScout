use anyhow::Result;
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
    pub fn new(furigana: &str) -> Result<UserNameFurigana> {
        if furigana.is_empty() {
            Err(UserNameFuriganaError::Empty.into())
        } else if furigana.chars().count() > 50 {
            Err(UserNameFuriganaError::TooLong.into())
        } else {
            Ok(UserNameFurigana(furigana.to_string()))
        }
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for UserNameFurigana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for UserNameFurigana {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod test_user_name_furigana {
    use super::*;

    #[test]
    fn ok() {
        let furigana = UserNameFurigana::new("イシダケンタロウ").unwrap();
        assert_eq!(furigana.to_string(), "イシダケンタロウ");
    }

    #[test]
    fn empty() {
        let furigana = UserNameFurigana::new("");
        assert_eq!(furigana.is_err(), true);
    }

    #[test]
    fn too_long() {
        let furigana = UserNameFurigana::new("パブロ・ディエーゴ・ホセ・フランシスコ・デ・パウラ・ホアン・ネポムセーノ・マリーア・デ・ロス・レメディオス・クリスピーン・クリスピアーノ・デ・ラ・サンティシマ・トリニダード・ルイス・イ・ピカソ");
        assert_eq!(furigana.is_err(), true);
    }

    #[test]
    fn length_50() {
        let furigana = UserNameFurigana::new("パブロ・ディエーゴ・ホセ・フランシスコ・デ・パウラ・ホアン・ネポムセーノ・マリーア・デ・ロス・レメデ");
        assert_eq!(furigana.is_ok(), true);
    }

    #[test]
    fn length_51() {
        let furigana = UserNameFurigana::new("パブロ・ディエーゴ・ホセ・フランシスコ・デ・パウラ・ホアン・ネポムセーノ・マリーア・デ・ロス・レメディ");
        assert_eq!(furigana.is_err(), true);
    }
}
