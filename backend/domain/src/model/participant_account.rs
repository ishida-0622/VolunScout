use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::{gender::Gender, region::Region, user_account::User, volunteer::Volunteer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user: User,
    pub gender: Gender,
    pub birthday: NaiveDate,
    pub region: Vec<Region>,
    pub profile: String,
    pub favorites: Vec<Volunteer>,
    pub activities: Vec<Volunteer>,
    pub scheduled_activities: Vec<Volunteer>,
}

impl Participant {
    pub fn new(
        user: User,
        gender: Gender,
        birthday: NaiveDate,
        region: Vec<Region>,
        profile: String,
    ) -> Participant {
        Participant {
            user,
            gender,
            birthday,
            region,
            profile,
            activities: Vec::new(),
            favorites: Vec::new(),
            scheduled_activities: Vec::new(),
        }
    }
}
