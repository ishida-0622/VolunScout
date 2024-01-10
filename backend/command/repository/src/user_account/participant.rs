use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDate;

use domain::model::{
    gender::Gender,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    }, terms::Terms
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
        profile: String,
        terms: Terms
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
        profile: String,
        terms: Terms
    ) -> Result<()>;

    /// 参加者アカウントを削除する
    async fn delete(&self, pid: UserId) -> Result<()>;
}
