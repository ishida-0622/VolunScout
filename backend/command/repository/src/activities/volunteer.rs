use anyhow::Result;
use async_trait::async_trait;

use chrono::{DateTime, NaiveDate, Utc};
use domain::model::{
    volunteer::VolunteerId,
    user_account::user_id::UserId,
    terms::Terms
};


#[async_trait]
pub trait VolunteerRepository: Send + Sync {
    /// ボランティアを作成する
    async fn create(
        &self,
        vid: VolunteerId,
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
        reward: Option<String>,
        terms: Terms,
        photo_keys: Vec<String>
    ) -> Result<()>;

    /// ボランティアを更新する
    async fn update(
        &self,
        vid: VolunteerId,
        title: String,
        message: String,
        overview: String,
        recruited_num: u32,
        place: String,
        start_at: DateTime<Utc>,
        finish_at: DateTime<Utc>,
        deadline_on: NaiveDate,
        as_group: bool,
        reward: Option<String>,
        terms: Terms,
        photo_keys: Vec<String>
    ) -> Result<()>;

    /// ボランティアを削除する
    async fn delete(&self, vid: VolunteerId) -> Result<()>;

    // お気に入りに登録する
    async fn register_favorite(&self, uid: UserId, vid: VolunteerId) -> Result<()>;

    // お気に入りの登録を解除する
    async fn unregister_favorite(&self, uid: UserId, vid: VolunteerId) -> Result<()>;
}
