use anyhow::Result;
use async_trait::async_trait;

use domain::model::user_account::{
    user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
    user_phone::UserPhone,
};

#[async_trait]
pub trait GroupUserRepository: Send + Sync {
    /// 団体アカウントを作成する
    async fn create(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        representative_name: UserName,
        representative_furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
        photo_keys: Vec<String>
    ) -> Result<()>;

    /// 団体アカウントを更新する
    async fn update(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        representative_name: UserName,
        representative_furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
        photo_keys: Vec<String>
    ) -> Result<()>;

    /// 団体アカウントの有償・無償プランを変更する
    async fn switch_plan(
        &self,
        gid: UserId,
        is_paid: bool
    ) -> Result<()>;

    /// 団体アカウントを削除する
    async fn delete(&self, gid: UserId) -> Result<()>;
}
