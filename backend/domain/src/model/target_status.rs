use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consts::target_status::{TargetStatusMap, TARGET_STATUSES, TARGET_STATUSES_PREFIX};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetStatus {
    target_status: String,
}

#[derive(Error, Debug)]
pub enum TargetStatusError {
    #[error("target_status not found")]
    NotFound,
}

impl TargetStatus {
    pub fn new(target_status: String) -> Result<TargetStatus> {
        for t in TARGET_STATUSES {
            if t == &target_status {
                return Ok(TargetStatus { target_status });
            }
        }
        Err(TargetStatusError::NotFound.into())
    }

    pub fn to_id(&self) -> String {
        let theme_map: TargetStatusMap = TargetStatusMap::new();
        let id = theme_map.target_statuses_name_to_index.get(&self.target_status).unwrap();
        id.to_string()
    }

    /// IDからprefixを除いたIDを取得する
    ///
    /// 例: theme_0 -> 0
    pub fn remove_prefix(&self) -> String {
        let theme_map: TargetStatusMap = TargetStatusMap::new();
        let id = theme_map.target_statuses_name_to_index.get(&self.target_status).unwrap();
        id.replace(TARGET_STATUSES_PREFIX, "")
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for TargetStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.target_status)
    }
}

// FromStrを実装することで, from_str()で文字列から変換できるようになる
impl std::str::FromStr for TargetStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TargetStatus::new(s.to_string())
    }
}
