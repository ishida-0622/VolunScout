pub mod events;

use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::model::{terms::Terms, user_account::UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volunteer {
    pub id: VolunteerId,
    pub gid: UserId,
    pub overview: String,
    pub people_num: u32,
    pub as_group: bool,
    pub start_at: DateTime<Utc>,
    pub start_day: i8,
    pub finish_at: DateTime<Utc>,
    pub finish_day: i8,
    pub deadline_on: NaiveDate,
    pub terms: Terms,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerId(pub ULID);

impl VolunteerId {
    pub fn new() -> VolunteerId {
        let mut generator: ULIDGenerator = ULIDGenerator::new();
        let value: ULID = generator.generate().unwrap();
        VolunteerId(value)
    }

    pub fn from_str(value: &str) -> VolunteerId {
        let id = ULID::from_str(&value).unwrap();
        VolunteerId(id)
    }
}

impl Volunteer {
    pub fn new(
        gid: UserId,
        overview: String,
        people_num: u32,
        as_group: bool,
        start_at: DateTime<Utc>,
        start_day: i8,
        finish_at: DateTime<Utc>,
        finish_day: i8,
        deadline_on: NaiveDate,
        terms: Terms,
    ) -> Volunteer {
        let id: VolunteerId = VolunteerId::new();
        Volunteer {
            id,
            gid,
            overview,
            people_num,
            as_group,
            start_at,
            start_day,
            finish_at,
            finish_day,
            deadline_on,
            terms,
        }
    }
}
