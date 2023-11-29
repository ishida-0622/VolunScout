use std::collections::HashMap;

pub const TARGET_STATUSES: [&str; 5] = [
    "社会人",
    "大学生・専門学校生",
    "高校生",
    "小中学生",
    "シニア",
];

#[derive(Debug, Clone)]
pub struct TargetStatusMap {
    pub target_statuses_index_to_name: HashMap<usize, String>,
    pub target_statuses_name_to_index: HashMap<String, usize>,
}

impl TargetStatusMap {
    pub fn new() -> TargetStatusMap {
        let target_statuses_index_to_name: HashMap<usize, String> = TARGET_STATUSES
            .iter()
            .enumerate()
            .map(|(index, target_status)| (index, target_status.to_string()))
            .collect::<HashMap<usize, String>>();
        let target_statuses_name_to_index: HashMap<String, usize> = TARGET_STATUSES
            .iter()
            .enumerate()
            .map(|(index, target_status)| (target_status.to_string(), index))
            .collect::<HashMap<String, usize>>();
        TargetStatusMap {
            target_statuses_index_to_name,
            target_statuses_name_to_index,
        }
    }
}
