use std::collections::HashMap;

pub const REGIONS: [&str; 53] = [
    "オンライン",
    "海外",
    "道北",
    "道東",
    "道南",
    "道央",
    "青森県",
    "岩手県",
    "宮城県",
    "秋田県",
    "山形県",
    "福島県",
    "茨城県",
    "栃木県",
    "群馬県",
    "埼玉県",
    "千葉県",
    "東京都23区",
    "東京都23区外",
    "神奈川県",
    "新潟県",
    "富山県",
    "石川県",
    "福井県",
    "山梨県",
    "長野県",
    "岐阜県",
    "静岡県",
    "愛知県",
    "三重県",
    "滋賀県",
    "京都府",
    "大阪府",
    "兵庫県",
    "奈良県",
    "和歌山県",
    "鳥取県",
    "島根県",
    "岡山県",
    "広島県",
    "山口県",
    "徳島県",
    "香川県",
    "愛媛県",
    "高知県",
    "福岡県",
    "佐賀県",
    "長崎県",
    "熊本県",
    "大分県",
    "宮崎県",
    "鹿児島県",
    "沖縄県",
];

#[derive(Debug, Clone)]
pub struct RegionMap {
    pub regions_index_to_name: HashMap<usize, String>,
    pub regions_name_to_index: HashMap<String, usize>,
}

impl RegionMap {
    pub fn new() -> RegionMap {
        let regions_index_to_name: HashMap<usize, String> = REGIONS
            .iter()
            .enumerate()
            .map(|(index, region)| (index, region.to_string()))
            .collect::<HashMap<usize, String>>();
        let regions_name_to_index: HashMap<String, usize> = REGIONS
            .iter()
            .enumerate()
            .map(|(index, region)| (region.to_string(), index))
            .collect::<HashMap<String, usize>>();
        RegionMap {
            regions_index_to_name,
            regions_name_to_index,
        }
    }
}
