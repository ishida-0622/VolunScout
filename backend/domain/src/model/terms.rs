use serde::{Deserialize, Serialize};

use crate::model::{region::Region, target_status::TargetStatus, theme::Theme, condition::Condition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub regions: Vec<Region>,
    pub themes: Vec<Theme>,
    pub required_themes: Vec<Theme>,
    pub conditions: Vec<Condition>,
    pub required_conditions: Vec<Condition>,
    pub reward: Option<String>,
    pub target_status: Vec<TargetStatus>,
}

impl Terms {
    pub fn new(
        regions: Vec<Region>,
        themes: Vec<Theme>,
        required_themes: Vec<Theme>,
        conditions: Vec<Condition>,
        required_conditions: Vec<Condition>,
        reward: Option<String>,
        target_status: Vec<TargetStatus>,
    ) -> Terms {
        Terms {
            regions,
            themes,
            required_themes,
            conditions,
            required_conditions,
            reward,
            target_status,
        }
    }
}
