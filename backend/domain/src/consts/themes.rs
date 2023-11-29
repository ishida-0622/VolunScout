use std::collections::HashMap;

pub const THEMES: [&str; 10] = [
    "国際",
    "教育・こども",
    "災害支援",
    "地域活性化",
    "イベント運営",
    "福祉・高齢者・障がい",
    "文化・芸術・スポーツ",
    "自然・農業",
    "動物愛護",
    "その他",
];

pub const THEMES_PREFIX: &str = "theme_";

#[derive(Debug, Clone)]
pub struct ThemeMap {
    pub themes_id_to_name: HashMap<String, String>,
    pub themes_name_to_id: HashMap<String, String>,
}

impl ThemeMap {
    pub fn new() -> ThemeMap {
        let themes_id_to_name: HashMap<String, String> = THEMES
            .iter()
            .enumerate()
            .map(|(index, theme)| (format!("{}{}", THEMES_PREFIX, index), theme.to_string()))
            .collect();
        let themes_name_to_id: HashMap<String, String> = THEMES
            .iter()
            .enumerate()
            .map(|(index, theme)| (theme.to_string(), format!("{}{}", THEMES_PREFIX, index)))
            .collect();
        ThemeMap {
            themes_id_to_name,
            themes_name_to_id,
        }
    }
}
