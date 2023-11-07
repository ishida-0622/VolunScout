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

#[derive(Debug, Clone)]
pub struct ThemeMap {
    pub themes_index_to_theme: HashMap<usize, String>,
    pub themes_theme_to_index: HashMap<String, usize>,
}

impl ThemeMap {
    pub fn new() -> ThemeMap {
        let themes_index_to_theme: HashMap<usize, String> = THEMES
            .iter()
            .enumerate()
            .map(|(index, theme)| (index, theme.to_string()))
            .collect::<HashMap<usize, String>>();
        let themes_theme_to_index: HashMap<String, usize> = THEMES
            .iter()
            .enumerate()
            .map(|(index, theme)| (theme.to_string(), index))
            .collect::<HashMap<String, usize>>();
        ThemeMap {
            themes_index_to_theme,
            themes_theme_to_index,
        }
    }
}
