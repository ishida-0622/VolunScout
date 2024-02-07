use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

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
    start_at: NaiveDateTime,
    finish_at: NaiveDateTime,
    deadline_on: NaiveDate,
    as_group: bool,
    is_deleted: bool,
    deleted_at: Option<NaiveDateTime>,
    registered_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    regions: Vec<String>,
    themes: Vec<String>,
    required_themes: Vec<String>,
    conditions: Vec<String>,
    required_conditions: Vec<String>,
    target_status: Vec<String>,
    photo_urls: Vec<String>,
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
        start_at: NaiveDateTime,
        finish_at: NaiveDateTime,
        deadline_on: NaiveDate,
        as_group: bool,
        is_deleted: bool,
        deleted_at: Option<NaiveDateTime>,
        registered_at: NaiveDateTime,
        updated_at: NaiveDateTime,
        regions: Vec<String>,
        themes: Vec<String>,
        required_themes: Vec<String>,
        conditions: Vec<String>,
        required_conditions: Vec<String>,
        target_status: Vec<String>,
        photo_urls: Vec<String>,
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
            photo_urls,
        }
    }
}

/// ボランティア要素類リードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct VolunteerElementsReadModel {
    pub vid: String,
    pub regions: Vec<String>,
    pub themes: Vec<String>,
    pub required_themes: Vec<String>,
    pub conditions: Vec<String>,
    pub required_conditions: Vec<String>,
    pub target_status: Vec<String>,
}

impl VolunteerElementsReadModel {
    pub fn new(
        vid: String,
        regions: Vec<String>,
        themes: Vec<String>,
        required_themes: Vec<String>,
        conditions: Vec<String>,
        required_conditions: Vec<String>,
        target_status: Vec<String>,
    ) -> VolunteerElementsReadModel {
        VolunteerElementsReadModel {
            vid,
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
    /// ボランティアに関連する要素をボランティアIDから取得する
    async fn find_elements_by_id(&self, vid: &VolunteerId) -> Result<VolunteerElementsReadModel>;

    /// ボランティアをボランティアidで取得する
    async fn find_by_id(&self, vid: &VolunteerId) -> Result<VolunteerReadModel>;

    /// ボランティアをグループidで取得する
    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<VolunteerReadModel>>;

    /// 参加者のお気に入りを取得する
    async fn find_favorite_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>>;

    /// 参加者の活動履歴を取得する
    async fn find_activity_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>>;

    /// 参加者の予定を取得する
    async fn find_scheduled_activity_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>>;
}
