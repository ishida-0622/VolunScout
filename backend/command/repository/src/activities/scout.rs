use anyhow::Result;
use async_trait::async_trait;

use domain::model::{
    volunteer::VolunteerId,
    user_account::user_id::UserId, scout::ScoutId
};


#[async_trait]
pub trait ScoutRepository: Send + Sync {
    /// スカウトする
    async fn create(
        &self,
        sid: ScoutId,
        vid: VolunteerId,
        user_id: UserId,
        message: String
    ) -> Result<()>;

    // スカウトメールの送信を送信済みにする
    async fn update_is_sent(&self, sid: ScoutId) -> Result<()>;

    /// スカウトを既読状態にする
    async fn update_is_read(&self, sid: ScoutId) -> Result<()>;

    // スカウトを辞退する
    async fn update_denied(&self, sid: ScoutId) -> Result<()>;

}
