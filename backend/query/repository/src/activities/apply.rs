use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::NaiveDateTime;

use domain::model::{user_account::user_id::UserId, apply::ApplyId, volunteer::VolunteerId};

/// スカウトリードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct Apply {
    /// 応募ID
    pub aid: String,
    /// ボランティアID
    pub vid: String,
    /// 参加者ID
    pub uid: String,
    /// 応募日時
    pub applied_at: NaiveDateTime,
    /// 集団応募有無
    pub as_group: bool,
    /// 認証データ
    pub allowed_status: i8,
    /// 認証日時
    pub decided_at: Option<NaiveDateTime>,
    /// 送信日時
    pub is_sent: bool
}

impl Apply {
    pub fn new(
        aid: String,
        vid: String,
        uid: String,
        applied_at: NaiveDateTime,
        as_group: bool,
        allowed_status: i8,
        decided_at: Option<NaiveDateTime>,
        is_sent: bool
    ) -> Apply {
        Apply {
            aid,
            vid,
            uid,
            applied_at,
            as_group,
            allowed_status,
            decided_at,
            is_sent,
        }
    }
}

#[async_trait]
pub trait ApplyRepository: Send + Sync {
    /// 応募情報を応募IDで取得する
    async fn find_by_sid(&self, sid: &ApplyId) -> Result<Apply>;

    /// 応募情報を団体IDで一括取得する
    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<Apply>>;

    /// 応募情報を参加者IDで一括取得する
    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Apply>>;

    /// 応募情報をボランティアIDで一括取得する
    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Apply>>;
}
