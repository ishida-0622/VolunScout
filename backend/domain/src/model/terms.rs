use serde::{Deserialize, Serialize};

use crate::model::{region::Region, target_status::TargetStatus, theme::Theme};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub regions: Vec<Region>,
    pub themes: Vec<Theme>,
    pub transportation_expenses: bool,
    pub reward: Option<String>,
    pub target_status: Vec<TargetStatus>,
}

impl Terms {
    pub fn new(
        regions: Vec<Region>,
        themes: Vec<Theme>,
        transportation_expenses: bool,
        reward: Option<String>,
        target_status: Vec<TargetStatus>,
    ) -> Terms {
        Terms {
            regions,
            themes,
            transportation_expenses,
            reward,
            target_status,
        }
    }
}
