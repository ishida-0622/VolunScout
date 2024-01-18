use serde::{Deserialize, Serialize};

use crate::model::{
    condition::Condition, region::Region, target_status::TargetStatus, theme::Theme,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub regions: Vec<Region>,
    pub themes: Vec<Theme>,
    pub required_themes: Vec<Theme>,
    pub conditions: Vec<Condition>,
    pub required_conditions: Vec<Condition>,
    pub target_status: TargetStatus,
}

impl Terms {
    pub fn new(
        regions: Vec<Region>,
        themes: Vec<Theme>,
        required_themes: Vec<Theme>,
        conditions: Vec<Condition>,
        required_conditions: Vec<Condition>,
        target_status: TargetStatus,
    ) -> Terms {
        Terms {
            regions,
            themes,
            required_themes,
            conditions,
            required_conditions,
            target_status,
        }
    }
}
