use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::model::{user_account::user_id::UserId, volunteer::VolunteerId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub uid: UserId,
    pub vid: VolunteerId,
    pub registered_at: DateTime<Utc>,
}

impl Favorite {
    pub fn new(uid: UserId, vid: VolunteerId) -> Favorite {
        Favorite {
            uid,
            vid,
            registered_at: Utc::now()
        }
    }
}
