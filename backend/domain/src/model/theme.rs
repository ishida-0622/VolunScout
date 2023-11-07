use serde::{Deserialize, Serialize};

use crate::consts::themes::THEMES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub theme: String,
}

#[derive(Debug)]
pub struct ThemeNotFoundError;

impl Theme {
    pub fn new(theme: String) -> Result<Theme, ThemeNotFoundError> {
        for t in THEMES {
            if t == &theme {
                return Ok(Theme { theme });
            }
        }
        Err(ThemeNotFoundError)
    }
}
