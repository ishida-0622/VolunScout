pub mod events;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::model::{user_account::user_id::UserId, volunteer::VolunteerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apply {
    pub id: ApplyId,
    pub user_id: UserId,
    pub volunteer_id: VolunteerId,
    pub people_num: u32,
    pub apply_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyId(pub ULID);

impl ApplyId {
    pub fn new() -> ApplyId {
        let mut generator: ULIDGenerator = ULIDGenerator::new();
        let value: ULID = generator.generate().unwrap();
        ApplyId(value)
    }
}

impl Apply {
    pub fn new(user_id: UserId, volunteer_id: VolunteerId, people_num: u32) -> Apply {
        let apply_id: ApplyId = ApplyId::new();
        let apply_at: DateTime<Utc> = Utc::now();
        Apply {
            id: apply_id,
            user_id,
            volunteer_id,
            people_num,
            apply_at,
        }
    }
}

impl std::fmt::Display for ApplyId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
