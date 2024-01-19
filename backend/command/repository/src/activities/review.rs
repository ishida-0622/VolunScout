use anyhow::Result;
use async_trait::async_trait;

use domain::model::{
    user_account::user_id::UserId, volunteer::VolunteerId
};

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    /// 参加者がボランティアにレビューする
    async fn review_to_volunteer(
        &self,
        uid: UserId,
        vid: VolunteerId,
        point: u8,
        comment: Option<String>
    ) -> Result<()>;

    /// ボランティア(各ボランティアの団体)が参加者にレビューする
    async fn review_to_participant(
        &self,
        uid: UserId,
        vid: VolunteerId,
        point: u8,
        comment: Option<String>
    ) -> Result<()>;
}
