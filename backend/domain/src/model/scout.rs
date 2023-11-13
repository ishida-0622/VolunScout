use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::model::{user_account::user_id::UserId, volunteer::VolunteerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scout {
    pub id: ScoutId,
    pub destination: UserId,
    pub sender: VolunteerId,
    pub message: String,
    pub send_at: DateTime<Utc>,
    pub is_send: bool,
    pub is_read: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutId(pub ULID);

impl ScoutId {
    pub fn new() -> ScoutId {
        let mut generator: ULIDGenerator = ULIDGenerator::new();
        let value: ulid_generator_rs::ULID = generator.generate().unwrap();
        ScoutId(value)
    }
}

impl Scout {
    pub fn new(destination: UserId, sender: VolunteerId, message: String) -> Scout {
        let id: ScoutId = ScoutId::new();
        let send_at: DateTime<Utc> = Utc::now();
        Scout {
            id,
            destination,
            sender,
            message,
            send_at,
            is_send: false,
            is_read: false,
        }
    }
}
