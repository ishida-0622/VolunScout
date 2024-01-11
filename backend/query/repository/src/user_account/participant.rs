use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

use domain::model::{user_account::user_id::UserId, volunteer::Volunteer};

use crate::MySqlBool;

/// 参加者アカウントリードモデル
#[derive(SimpleObject)]
pub struct ParticipantAccount {
    /// 参加者アカウントid
    pub uid: String,
    /// 参加者氏名
    pub name: String,
    /// 参加者氏名(フリガナ)
    pub furigana: String,
    /// 電話番号
    pub phone: String,
    /// 性別
    ///
    /// 0: 男性, 1: 女性, 2: その他
    pub gender: i8,
    /// 生年月日
    pub birthday: NaiveDate,
    /// プロフィール
    pub profile: String,
    /// 削除フラグ
    pub is_deleted: MySqlBool,
    /// 削除日時
    pub deleted_at: Option<NaiveDateTime>,
}

/// 参加者地域リードモデル
#[derive(SimpleObject)]
pub struct ParticipantRegion {
    /// 地域名
    pub name: String,
}

/// 参加者テーマリードモデル
#[derive(SimpleObject)]
pub struct ParticipantTheme {
    /// テーマ名
    pub name: String,
    /// 必須フラグ
    pub is_required: MySqlBool,
}

/// 参加者条件リードモデル
#[derive(SimpleObject)]
pub struct ParticipantCondition {
    /// 条件名
    pub name: String,
    /// 必須フラグ
    pub is_required: MySqlBool,
}

#[async_trait]
pub trait ParticipantUserRepository: Send + Sync {
    /// 参加者アカウントをIDで取得する
    async fn find_by_id(&self, pid: &UserId) -> Result<ParticipantAccount>;

    /// 参加者アカウントをIDで複数取得する
    async fn find_by_ids(&self, pids: &[UserId]) -> Result<Vec<ParticipantAccount>>;

    /// 参加者の活動地域を取得する
    async fn find_region_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantRegion>>;

    /// 参加者のテーマを取得する
    async fn find_theme_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantTheme>>;

    /// 参加者の条件を取得する
    async fn find_condition_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantCondition>>;

    /// 参加者のお気に入りを取得する
    async fn find_favorite_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;

    /// 参加者の活動履歴を取得する
    async fn find_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;

    /// 参加者の予定を取得する
    async fn find_scheduled_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;

    /// 参加者が存在するか確認する
    async fn exists(&self, pid: &UserId) -> Result<bool>;
}
