use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consts::themes::{THEMES, ThemeMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    theme: String,
}

#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("theme not found")]
    NotFound,
}

impl Theme {
    pub fn new(theme: String) -> Result<Theme> {
        for t in THEMES {
            if t == &theme {
                return Ok(Theme { theme });
            }
        }
        Err(ThemeError::NotFound.into())
    }

    pub fn to_uint(&self) -> u8 {
        let theme_map: ThemeMap = ThemeMap::new();
        let index: &usize = theme_map.themes_name_to_index.get(&self.theme).unwrap();
        *index as u8
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.theme)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for Theme {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Theme::new(s.to_string())
    }
}
