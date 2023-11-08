use serde::{Deserialize, Serialize};

use super::ScoutId;
use crate::model::user_account::user_id::UserId;
use crate::model::volunteer::VolunteerId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoutEvent {
    /// スカウトが作成された
    ScoutCreated(ScoutEventCreatedBody),
    /// スカウトが閲覧された
    ScoutRead(ScoutId),
    /// スカウトが辞退された
    ScoutDeclined(ScoutEventDeclinedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutEventCreatedBody {
    pub destination: UserId,
    pub sender: VolunteerId,
    pub message: String,
}

impl ScoutEventCreatedBody {
    pub fn new(destination: UserId, sender: VolunteerId, message: String) -> ScoutEventCreatedBody {
        Self {
            destination,
            sender,
            message,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutEventDeclinedBody {
    pub id: ScoutId,
}
