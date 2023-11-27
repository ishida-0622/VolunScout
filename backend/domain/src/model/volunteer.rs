use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use crate::model::{terms::Terms, user_account::user_id::UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volunteer {
    pub vid: VolunteerId,
    pub gid: UserId,
    pub title: String,
    pub message: String,
    pub overview: String,
    pub recruited_num: u32,
    pub place: String,
    pub start_at: DateTime<Utc>,
    pub finish_at: DateTime<Utc>,
    pub deadline_on: NaiveDate,
    pub as_group: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub registerd_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
        let vid = ULID::from_str(&value).unwrap();
        VolunteerId(vid)
    }
}
// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for VolunteerId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl Volunteer {
    pub fn new(
        gid: UserId,
        title: String,
        message: String,
        overview: String,
        recruited_num: u32,
        place: String,
        start_at: DateTime<Utc>,
        finish_at: DateTime<Utc>,
        deadline_on: NaiveDate,
        as_group: bool,
        is_deleted: bool,
        deleted_at: Option<DateTime<Utc>>,
        registerd_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        terms: Terms,
    ) -> Volunteer {
        let vid: VolunteerId = VolunteerId::new();
        Volunteer {
            vid,
            gid,
            title,
            message,
            overview,
            recruited_num,
            place,
            start_at,
            finish_at,
            deadline_on,
            as_group,
            is_deleted,
            deleted_at,
            registerd_at,
            updated_at,
            terms,
        }
    }
}
