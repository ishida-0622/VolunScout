use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::{gender::Gender, region::Region, user_account::User, volunteer::Volunteer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub user: User,
    pub gender: Gender,
    pub birthday: NaiveDate,
    pub regions: Vec<Region>,
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
        regions: Vec<Region>,
        profile: String,
    ) -> Participant {
        Participant {
            user,
            gender,
            birthday,
            regions,
            profile,
            activities: Vec::new(),
            favorites: Vec::new(),
            scheduled_activities: Vec::new(),
        }
    }
}
