use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;

use domain::model::{
    condition::Condition,
    gender::Gender,
    region::Region,
    theme::Theme,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
};

#[async_trait]
pub trait ParticipantUserRepository: Send + Sync {
    /// 参加者アカウントを作成する
    async fn create(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        regions: Vec<Region>,
        profile: String,
        themes: Vec<Theme>,
        themes_required: Vec<Theme>,
        conditions: Vec<Condition>,
        conditions_required: Vec<Condition>,
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
        regions: Vec<Region>,
        profile: String,
        theme: Vec<Theme>,
        theme_required: Vec<Theme>,
        conditions: Vec<Condition>,
        conditions_required: Vec<Condition>,
    ) -> Result<()>;

    /// 参加者アカウントを削除する
    async fn delete(&self, pid: UserId) -> Result<()>;
}
