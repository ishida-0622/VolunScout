use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

use domain::model::{user_account::user_id::UserId, volunteer::VolunteerId};

/// ボランティアリードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct VolunteerReadModel {
    vid: String,
    gid: String,
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
    registered_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    regions: Vec<String>,
    themes: Vec<String>,
    required_themes: Vec<String>,
    conditions: Vec<String>,
    required_conditions: Vec<String>,
    target_status: Vec<String>,
}

impl VolunteerReadModel {
    pub fn new(
        vid: String,
        gid: String,
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
        registered_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        regions: Vec<String>,
        themes: Vec<String>,
        required_themes: Vec<String>,
        conditions: Vec<String>,
        required_conditions: Vec<String>,
        target_status: Vec<String>,
    ) -> VolunteerReadModel {
        VolunteerReadModel {
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
            registered_at,
            updated_at,
            regions,
            themes,
            required_themes,
            conditions,
            required_conditions,
            target_status,
        }
    }
}

#[async_trait]
pub trait VolunteerQueryRepository: Send + Sync {
    /// ボランティアをボランティアidで取得する
    async fn find_by_id(&self, vid: &VolunteerId) -> Result<VolunteerReadModel>;

    /// ボランティアをグループidで取得する
    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<VolunteerReadModel>>;
}
