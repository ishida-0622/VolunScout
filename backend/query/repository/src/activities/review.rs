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
    pub comment: Option<String>,
}

/// レビューリードモデル（平均）
#[derive(SimpleObject, sqlx::Type)]
pub struct ParticipantReviewPointAverage {
    /// 参加者ID
    pub uid: String,
    /// レビューポイント
    pub point: f64,
}

impl Review {
    pub fn new(uid: String, vid: String, point: i8, comment: Option<String>) -> Review {
        Review {
            uid,
            vid,
            point,
            comment,
        }
    }
}

impl ParticipantReviewPointAverage {
    pub fn new(uid: String, point: f64) -> ParticipantReviewPointAverage {
        ParticipantReviewPointAverage { uid, point }
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

    /// 複数の参加者のレビュー情報を一括取得する
    async fn find_by_uids(&self, uids: &[UserId]) -> Result<Vec<ParticipantReviewPointAverage>>;
}

#[async_trait]
pub trait VolunteerReviewRepository: Send + Sync {
    /// ボランティアへのレビュー情報を参加者IDとボランティアIDで1件取得する
    async fn find_by_ids(&self, uid: &UserId, vid: &VolunteerId) -> Result<Review>;

    /// ボランティアへのレビュー情報を参加者IDで一括取得する
    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Review>>;

    /// ボランティアへのレビュー情報をボランティアIDで一括取得する
    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Review>>;

    /// ボランティアへのレビュー情報を団体IDで一括取得する
    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<Review>>;
}
