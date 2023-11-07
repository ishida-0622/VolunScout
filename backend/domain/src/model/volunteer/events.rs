use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::VolunteerId;
use crate::model::user_account::UserId;

pub enum VolunteerEvent {
    /// ボランティアが作成された
    VolunteerCreated(VolunteerEventCreatedBody),
    /// ボランティアが更新された
    VolunteerUpdated(VolunteerEventUpdatedBody),
    /// ボランティアが削除された
    VolunteerDeleted(VolunteerEventDeletedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerEventCreatedBody {
    pub gid: UserId,
    pub overview: String,
    pub people_num: i32,
    pub as_group: bool,
    pub start_at: DateTime<Utc>,
    pub finish_at: DateTime<Utc>,
    pub deadline_on: NaiveDate,
    pub regions: Vec<String>,
    pub themes: Vec<String>,
    pub transportation_expenses: bool,
    pub reward: String,
    pub target_status: Vec<String>,
}

impl VolunteerEventCreatedBody {
    pub fn new(
        gid: UserId,
        overview: String,
        people_num: i32,
        as_group: bool,
        start_at: DateTime<Utc>,
        finish_at: DateTime<Utc>,
        deadline_on: NaiveDate,
        regions: Vec<String>,
        themes: Vec<String>,
        transportation_expenses: bool,
        reward: String,
        target_status: Vec<String>,
    ) -> VolunteerEventCreatedBody {
        VolunteerEventCreatedBody {
            gid,
            overview,
            people_num,
            as_group,
            start_at,
            finish_at,
            deadline_on,
            regions,
            themes,
            transportation_expenses,
            reward,
            target_status,
        }
    }
}

// TODO:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerEventUpdatedBody {
    pub id: VolunteerId,
    pub overview: String,
    pub people_num: i32,
    pub as_group: bool,
    pub start_at: String,
    pub finish_at: String,
    pub deadline_at: String,
    pub regions: Vec<String>,
    pub themes: Vec<String>,
    pub transportation_expenses: bool,
    pub reward: String,
    pub target_status: Vec<String>,
}

impl VolunteerEventUpdatedBody {
    pub fn new(
        id: VolunteerId,
        overview: String,
        people_num: i32,
        as_group: bool,
        start_at: String,
        finish_at: String,
        deadline_at: String,
        regions: Vec<String>,
        themes: Vec<String>,
        transportation_expenses: bool,
        reward: String,
        target_status: Vec<String>,
    ) -> VolunteerEventUpdatedBody {
        VolunteerEventUpdatedBody {
            id,
            overview,
            people_num,
            as_group,
            start_at,
            finish_at,
            deadline_at,
            regions,
            themes,
            transportation_expenses,
            reward,
            target_status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerEventDeletedBody {
    pub id: VolunteerId,
}

impl VolunteerEventDeletedBody {
    pub fn new(id: VolunteerId) -> VolunteerEventDeletedBody {
        VolunteerEventDeletedBody { id }
    }
}
