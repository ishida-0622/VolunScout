use serde::{Deserialize, Serialize};

use super::ApplyId;
use crate::model::{user_account::UserId, volunteer::VolunteerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApplyEvent {
    /// 申請が作成された
    ApplyCreated(ApplyEventCreatedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyEventCreatedBody {
    pub id: ApplyId,
    pub user_id: UserId,
    pub volunteer_id: VolunteerId,
    pub people_num: u32,
    pub apply_at: String,
}

impl ApplyEventCreatedBody {
    pub fn new(
        id: ApplyId,
        user_id: UserId,
        volunteer_id: VolunteerId,
        people_num: u32,
        apply_at: String,
    ) -> ApplyEventCreatedBody {
        ApplyEventCreatedBody {
            id,
            user_id,
            volunteer_id,
            people_num,
            apply_at,
        }
    }
}
