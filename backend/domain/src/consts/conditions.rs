use std::collections::HashMap;

pub const CONDITIONS: [&str; 4] = [
    "交通費一部支給",
    "交通費全額支給",
    "報奨金あり",
    "学校法人向け",
];

pub const CONDITIONS_PREFIX: &str = "condition_";

#[derive(Debug, Clone)]
pub struct ConditionMap {
    pub conditions_id_to_name: HashMap<String, String>,
    pub conditions_name_to_id: HashMap<String, String>,
}

impl ConditionMap {
    pub fn new() -> ConditionMap {
        let conditions_id_to_name: HashMap<String, String> = CONDITIONS
            .iter()
            .enumerate()
            .map(|(index, condition)| {
                (
                    format!("{}{}", CONDITIONS_PREFIX, index),
                    condition.to_string(),
                )
            })
            .collect();
        let conditions_name_to_id: HashMap<String, String> = CONDITIONS
            .iter()
            .enumerate()
            .map(|(index, condition)| {
                (
                    condition.to_string(),
                    format!("{}{}", CONDITIONS_PREFIX, index),
                )
            })
            .collect();
        ConditionMap {
            conditions_id_to_name,
            conditions_name_to_id,
        }
    }
}
