use serde::{Deserialize, Serialize};

use crate::consts::target_status::TARGET_STATUSES;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetStatus {
    pub target_status: String,
}

#[derive(Debug)]
pub struct TargetStatusNotFoundError;

impl TargetStatus {
    pub fn new(target_status: String) -> Result<TargetStatus, TargetStatusNotFoundError> {
        for t in TARGET_STATUSES {
            if t == &target_status {
                return Ok(TargetStatus { target_status });
            }
        }
        Err(TargetStatusNotFoundError)
    }
}
