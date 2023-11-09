use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};

use super::MySqlBool;
use crate::model::{
    gender::Gender,
    region::Region,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
    volunteer::Volunteer,
};

// Read server で返す型. GraphQLのスキーマに対応する
/// 団体アカウントリードモデル
#[derive(SimpleObject)]
pub struct GroupAccount {
    /// 団体アカウントid
    pub gid: String,
    /// 団体名
    pub name: String,
    /// 団体名(フリガナ)
    pub furigana: String,
    /// 電話番号
    pub phone: String,
    /// 住所
    pub address: String,
    /// 団体紹介
    pub contents: String,
    /// 有料会員
    pub is_paid: MySqlBool,
    /// 削除フラグ
    pub is_deleted: MySqlBool,
    /// 削除日時
    pub deleted_at: Option<NaiveDateTime>,
}

impl GroupAccount {
    pub fn new(
        gid: String,
        name: String,
        furigana: String,
        phone: String,
        address: String,
        contents: String,
        is_paid: MySqlBool,
        is_deleted: MySqlBool,
        deleted_at: Option<NaiveDateTime>,
    ) -> GroupAccount {
        GroupAccount {
            gid,
            name,
            furigana,
            phone,
            address,
            contents,
            is_paid,
            is_deleted,
            deleted_at,
        }
    }
}

#[async_trait]
pub trait GroupUserRepository: Send + Sync {
    /// 団体アカウントを作成する
    async fn create(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
    ) -> Result<()>;

    /// 団体アカウントを更新する
    async fn update(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
    ) -> Result<()>;

    /// 団体アカウントを削除する
    async fn delete(&self, gid: UserId) -> Result<()>;

    /// 団体アカウントをIDで取得する
    async fn find_by_id(&self, gid: &UserId) -> Result<GroupAccount>;

    /// 団体アカウントをIDで複数取得する
    async fn find_by_ids(&self, gids: &[UserId]) -> Result<Vec<GroupAccount>>;

    /// 団体アカウントを全て取得する
    async fn find_all(&self) -> Result<Vec<GroupAccount>>;
}

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

#[async_trait]
pub trait ParticipantsUserRepository: Send + Sync {
    /// 参加者アカウントを作成する
    async fn create(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        region: Vec<Region>,
        profile: String,
    ) -> Result<()>;

    /// 参加者アカウントを更新する
    async fn update(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        region: Vec<Region>,
        profile: String,
    ) -> Result<()>;

    /// 参加者アカウントを削除する
    async fn delete(&self, pid: UserId) -> Result<()>;

    /// 参加者アカウントをIDで取得する
    async fn find_by_id(&self, pid: &UserId) -> Result<ParticipantAccount>;

    /// 参加者アカウントをIDで複数取得する
    async fn find_by_ids(&self, pids: &[UserId]) -> Result<Vec<ParticipantAccount>>;

    /// 参加者の活動地域を取得する
    async fn find_region_by_id(&self, pid: &UserId) -> Result<Vec<Region>>;

    /// 参加者のお気に入りを取得する
    async fn find_favorite_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;

    /// 参加者の活動履歴を取得する
    async fn find_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;

    /// 参加者の予定を取得する
    async fn find_scheduled_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>>;
}
