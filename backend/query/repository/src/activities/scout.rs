use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

use domain::model::{scout::ScoutId, user_account::user_id::UserId, volunteer::VolunteerId};

/// スカウトリードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct ScoutFromGroup {
    // スカウトID
    pub sid: String,
    // ボランティアID
    pub vid: String,
    // 参加者ID
    pub uid: String,
    // スカウトメッセージ
    pub message: String,
    // スカウト日時
    pub scouted_at: NaiveDateTime,
    // 既読有無
    pub is_read: bool,
    // スカウトメール送信有無
    pub is_sent: bool,
    // スカウトメール送信日時
    pub sent_at: Option<NaiveDateTime>,
    // 辞退有無
    pub is_denied: bool,
    // 辞退日時
    pub denied_at: Option<NaiveDateTime>,

    pub name: String,
    pub gender: u8,
    pub birthday: NaiveDate,
    pub point: f32,
}

#[derive(SimpleObject, sqlx::Type)]
pub struct Scout {
    // スカウトID
    pub sid: String,
    // ボランティアID
    pub vid: String,
    // 参加者ID
    pub uid: String,
    // スカウトメッセージ
    pub message: String,
    // スカウト日時
    pub scouted_at: NaiveDateTime,
    // 既読有無
    pub is_read: bool,
    // スカウトメール送信有無
    pub is_sent: bool,
    // スカウトメール送信日時
    pub sent_at: Option<NaiveDateTime>,
    // 辞退有無
    pub is_denied: bool,
    // 辞退日時
    pub denied_at: Option<NaiveDateTime>,
}

impl Scout {
    pub fn new(
        sid: String,
        vid: String,
        uid: String,
        message: String,
        scouted_at: NaiveDateTime,
        is_read: bool,
        is_sent: bool,
        sent_at: Option<NaiveDateTime>,
        is_denied: bool,
        denied_at: Option<NaiveDateTime>,
    ) -> Scout {
        Scout {
            sid,
            vid,
            uid,
            message,
            scouted_at,
            is_read,
            is_sent,
            sent_at,
            is_denied,
            denied_at,
        }
    }
}

#[async_trait]
pub trait ScoutRepository: Send + Sync {
    /// スカウト情報をスカウトIDで取得する
    async fn find_by_sid(&self, sid: &ScoutId) -> Result<Scout>;

    /// スカウト情報を団体IDで一括取得する
    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<Scout>>;

    /// スカウト情報を参加者IDで一括取得する
    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Scout>>;

    /// スカウト情報をボランティアIDで一括取得する
    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<ScoutFromGroup>>;
}
