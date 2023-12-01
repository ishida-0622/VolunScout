pub mod user_id;
pub mod user_name;
pub mod user_name_furigana;
pub mod user_phone;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use self::{
    user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
    user_phone::UserPhone,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub phone: UserPhone,
    pub review: Review,
    pub review_relation: Vec<ReviewRelation>,
    pub is_deleted: bool,
    pub deleted_on: Option<NaiveDate>,
}

impl User {
    pub fn new(id: UserId, name: UserName, furigana: UserNameFurigana, phone: UserPhone) -> User {
        User {
            id,
            name,
            furigana,
            phone,
            review: Review::new(),
            review_relation: Vec::new(),
            is_deleted: false,
            deleted_on: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub value: Option<f32>,
    one: i32,
    two: i32,
    three: i32,
    four: i32,
    five: i32,
}

impl Review {
    pub fn new() -> Review {
        Review {
            value: None,
            one: 0,
            two: 0,
            three: 0,
            four: 0,
            five: 0,
        }
    }

    pub fn add(&mut self, value: i32) {
        match value {
            1 => self.one += 1,
            2 => self.two += 1,
            3 => self.three += 1,
            4 => self.four += 1,
            5 => self.five += 1,
            _ => {}
        }
        let total: i32 = self.one + self.two + self.three + self.four + self.five;
        let value: f32 = (self.one + self.two * 2 + self.three * 3 + self.four * 4 + self.five * 5)
            as f32
            / total as f32;
        self.value = Some(value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRelation {
    pub to_id: super::ReviewToId,   // UserId or VolunteerId
    pub point: u8
}

impl ReviewRelation {
    pub fn new(to_id: super::ReviewToId, point: u8) -> ReviewRelation {
        ReviewRelation {
            to_id,
            point
        }
    }
}

#[test]
fn test_user_id() {
    let uid = UserId::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
    assert_eq!(uid.to_string(), "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    let uid = UserId::new("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    assert_eq!(uid.is_err(), true);
}
