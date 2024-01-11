use std::collections::HashMap;

pub const TARGET_STATUSES: [&str; 5] = [
    "社会人",
    "大学生・専門学校生",
    "高校生",
    "小中学生",
    "シニア",
];

pub const TARGET_STATUSES_PREFIX: &str = "target_status_";

#[derive(Debug, Clone)]
pub struct TargetStatusMap {
    pub target_statuses_index_to_name: HashMap<String, String>,
    pub target_statuses_name_to_index: HashMap<String, String>,
}

impl TargetStatusMap {
    pub fn new() -> TargetStatusMap {
        let target_statuses_index_to_name: HashMap<String, String> = TARGET_STATUSES
            .iter()
            .enumerate()
            .map(|(index, target_status)| (format!("{}{}", TARGET_STATUSES_PREFIX, index), target_status.to_string()))
            .collect();
        let target_statuses_name_to_index: HashMap<String, String> = TARGET_STATUSES
            .iter()
            .enumerate()
            .map(|(index, target_status)| (target_status.to_string(), format!("{}{}", TARGET_STATUSES_PREFIX, index)))
            .collect();
        TargetStatusMap {
            target_statuses_index_to_name,
            target_statuses_name_to_index,
        }
    }
}
