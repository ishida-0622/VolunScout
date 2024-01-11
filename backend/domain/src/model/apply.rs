use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::model::{user_account::user_id::UserId, volunteer::VolunteerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apply {
    pub aid: ApplyId,
    pub vid: VolunteerId,
    pub user_id: UserId,
    pub applied_at: DateTime<Utc>,
    pub as_group: bool,
    /// 0:未承認 1:承認済 2:棄却済
    pub allowed_status: u8,
    pub decided_at: Option<DateTime<Utc>>,
    pub is_sent: bool
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
    pub fn new(
        volunteer_id: VolunteerId,
        user_id: UserId,
        as_group: bool,
    ) -> Apply {
        let apply_id: ApplyId = ApplyId::new();
        let applied_at: DateTime<Utc> = Utc::now();
        Apply {
            aid: apply_id,
            vid: volunteer_id,
            user_id,
            applied_at,
            as_group,
            allowed_status: 0,
            decided_at: None,
            is_sent: false
        }
    }
}

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for ApplyId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
