use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;

use domain::model::{user_account::user_id::UserId, volunteer::VolunteerId};

/// レビューリードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct Review {
    /// 参加者ID
    pub uid: String,
    /// ボランティアID
    pub vid: String,
    /// レビューポイント
    pub point: i8,
    /// コメント
    pub comment: Option<String>
}

impl Review {
    pub fn new(
        uid: String,
        vid: String,
        point: i8,
        comment: Option<String>
    ) -> Review {
        Review {
            uid,
            vid,
            point,
            comment
        }
    }
}

#[async_trait]
pub trait ParticipantReviewRepository: Send + Sync {
    /// 参加者へのレビュー情報を参加者IDとボランティアIDで1件取得する
    async fn find_by_ids(&self, uid: &UserId, vid: &VolunteerId) -> Result<Review>;

    /// 参加者へのレビュー情報を参加者IDで一括取得する
    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Review>>;

    /// 参加者へのレビュー情報をボランティアIDで一括取得する
    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Review>>;
}

#[async_trait]
pub trait VolunteerReviewRepository: Send + Sync {
    /// ボランティアへのレビュー情報を参加者IDとボランティアIDで1件取得する
    async fn find_by_ids(&self, uid: &UserId, vid: &VolunteerId) -> Result<Review>;

    /// ボランティアへのレビュー情報を参加者IDで一括取得する
    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Review>>;

    /// ボランティアへのレビュー情報をボランティアIDで一括取得する
    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Review>>;
}
