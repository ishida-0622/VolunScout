use serde::{Deserialize, Serialize};

use crate::model::{user_account::user_id::UserId, volunteer::VolunteerId};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub uid: UserId,
    pub vid: VolunteerId,
    pub point: u8,
    pub comment: Option<String>
}

impl Review {
    pub fn new(
        uid: UserId,
        vid: VolunteerId,
        point: u8,
        comment: Option<String>
    ) -> Review {
        Review {
            uid,
            vid,
            point,
            comment
        }
    }
}
