use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

use domain::model::{apply::ApplyId, user_account::user_id::UserId};

use crate::activities::volunteer::VolunteerElementsReadModel;

/// 参加者アカウントリードモデル
#[derive(SimpleObject, sqlx::Type)]
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
    pub is_deleted: bool,
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
#[derive(SimpleObject, sqlx::Type)]
pub struct ParticipantTheme {
    /// テーマ名
    pub name: String,
    /// 必須フラグ
    pub is_required: bool,
}

/// 参加者条件リードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct ParticipantCondition {
    /// 条件名
    pub name: String,
    /// 必須フラグ
    pub is_required: bool,
}

/// 参加者区分リードモデル
#[derive(SimpleObject)]
pub struct ParticipantTargetStatus {
    pub name: String,
}

/// 集団応募者リードモデル
#[derive(SimpleObject)]
pub struct GroupParticipant {
    /// シリアル（連番）
    pub serial: u16,
    /// 名前
    pub name: String,
    /// フリガナ
    pub furigana: String,
    /// 性別
    ///
    /// 0: 男性, 1: 女性, 2: その他
    pub gender: u8,
    /// 年齢
    pub age: u8,
}

/// スカウト用参加者アカウントリードモデル
#[derive(SimpleObject, sqlx::Type)]
pub struct ScoutParticipant {
    /// 参加者アカウントid
    pub uid: String,
    /// 参加者氏名
    pub name: String,
    /// 性別
    ///
    /// 0: 男性, 1: 女性, 2: その他
    pub gender: i8,
    /// 生年月日
    pub birthday: NaiveDate,
    /// 平均評価値
    pub point: Option<f32>
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

    /// 参加者の区分を取得する
    async fn find_target_status_by_id(&self, pid: &UserId) -> Result<ParticipantTargetStatus>;

    /// 参加者が存在するか確認する
    async fn exists(&self, pid: &UserId) -> Result<bool>;

    /// 集団応募者の詳細情報を取得する
    async fn find_group_participants(&self, aid: &ApplyId) -> Result<Vec<GroupParticipant>>;

    /// 参加者を条件検索を用いて取得する
    async fn find_by_elements(
        &self,
        elements: &VolunteerElementsReadModel
    ) -> Result<Vec<ScoutParticipant>>;
}
