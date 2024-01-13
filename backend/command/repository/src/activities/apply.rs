use anyhow::Result;
use async_trait::async_trait;

use domain::model::{
    apply::ApplyId,
    volunteer::VolunteerId,
    user_account::user_id::UserId
};


#[async_trait]
pub trait ApplyRepository: Send + Sync {
    /// ボランティアに応募する
    async fn create(
        &self,
        aid: ApplyId,
        vid: VolunteerId,
        user_id: UserId,
        as_group: bool
    ) -> Result<()>;

    /// 応募の承認を更新する
    async fn update_allowed_status(
        &self,
        aid: ApplyId,
        allowed_status: u8,
    ) -> Result<()>;

    // 応募メールの送信を送信済みにする
    async fn update_is_sent(&self, aid: ApplyId) -> Result<()>;
}
