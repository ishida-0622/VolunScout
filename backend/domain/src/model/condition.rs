use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consts::conditions::{ConditionMap, CONDITIONS, CONDITIONS_PREFIX};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    condition: String,
}

#[derive(Error, Debug)]
pub enum ConditionError {
    #[error("condition not found")]
    NotFound,
}

impl Condition {
    pub fn new(condition: String) -> Result<Condition> {
        for c in CONDITIONS {
            if c == &condition {
                return Ok(Condition { condition });
            }
        }
        Err(ConditionError::NotFound.into())
    }

    pub fn to_id(&self) -> String {
        let condition_map: ConditionMap = ConditionMap::new();
        let id = condition_map
            .conditions_name_to_id
            .get(&self.condition)
            .unwrap();
        id.to_string()
    }

    /// IDからprefixを除いたIDを取得する
    ///
    /// 例: condition_0 -> 0
    pub fn remove_prefix(&self) -> String {
        let condition_map: ConditionMap = ConditionMap::new();
        let id = condition_map
            .conditions_name_to_id
            .get(&self.condition)
            .unwrap();
        id.replace(CONDITIONS_PREFIX, "")
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.condition)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Condition::new(s.to_string())
    }
}
