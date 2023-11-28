use serde::{Deserialize, Serialize};

use crate::consts::themes::{ThemeMap, THEMES, THEMES_PREFIX};

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

    pub fn to_id(&self) -> String {
        let theme_map: ThemeMap = ThemeMap::new();
        let id = theme_map.themes_name_to_id.get(&self.theme).unwrap();
        id.to_string()
    }

    /// IDからprefixを除いたIDを取得する
    ///
    /// 例: theme_0 -> 0
    pub fn remove_prefix(&self) -> String {
        let theme_map: ThemeMap = ThemeMap::new();
        let id = theme_map.themes_name_to_id.get(&self.theme).unwrap();
        id.replace(THEMES_PREFIX, "")
    }
}
