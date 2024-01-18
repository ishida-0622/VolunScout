use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;

use command_repository::activities::scout::ScoutRepository;
use domain::model::{
    scout::ScoutId,
    user_account::user_id::UserId, volunteer::VolunteerId, group_participants::GroupParticipants, gender::{gender_from_i8, gender_to_i8}
};

pub struct ScoutImpl {
    pool: MySqlPool,
}

impl ScoutImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ScoutRepository for ScoutImpl {
    async fn create(
        &self,
        sid: ScoutId,
        vid: VolunteerId,
        user_id: UserId,
        message: String
    ) -> Result<()> {
        let sid: String = sid.to_string();
        let vid: String = vid.to_string();
        let uid: String = user_id.to_string();

        sqlx::query!(
            "INSERT INTO scout (sid, vid, uid, message, scouted_at) VALUES (?, ?, ?, ?, ?)",
            sid,
            vid,
            uid,
            message,
            Utc::now()
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn update_is_sent(&self, sid: ScoutId) -> Result<()> {

        let sid: String = sid.to_string();
        sqlx::query!(
            "UPDATE scout SET is_sent = ?, sent_at = ? WHERE sid = ?",
            true,
            Utc::now(),
            sid
        )
        .execute(&self.pool).await?;
        Ok(())
    }

    async fn update_is_read(
        &self,
        sid: ScoutId
    ) -> Result<()> {
        let sid: String = sid.to_string();
        sqlx::query!(
            "UPDATE scout SET is_read = ? WHERE sid = ?",
            true,
            sid
        )
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn update_denied(
        &self,
        sid: ScoutId
    ) -> Result<()> {
        let sid: String = sid.to_string();
        sqlx::query!(
            "UPDATE scout SET is_denied = ?, denied_at = ? WHERE sid = ?",
            true,
            Utc::now(),
            sid
        )
        .execute(&self.pool).await?;

        Ok(())
    }
}
