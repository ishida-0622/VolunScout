use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use command_repository::activities::review::ReviewRepository;
use domain::model::{
    user_account::user_id::UserId, volunteer::VolunteerId
};

pub struct ReviewImpl {
    pool: MySqlPool,
}

impl ReviewImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReviewRepository for ReviewImpl {
    async fn review_to_volunteer(
        &self,
        uid: UserId,
        vid: VolunteerId,
        point: u8,
        comment: Option<String>
    ) -> Result<()> {
        let uid: String = uid.to_string();
        let vid: String = vid.to_string();

        sqlx::query!(
            "INSERT INTO volunteer_review (uid, vid, point, comment) VALUES (?, ?, ?, ?)",
            uid,
            vid,
            point,
            comment
        ).execute(&self.pool).await?;
        Ok(())
    }

    async fn review_to_participant(
        &self,
        uid: UserId,
        vid: VolunteerId,
        point: u8,
        comment: Option<String>
    ) -> Result<()> {
        let uid: String = uid.to_string();
        let vid: String = vid.to_string();

        sqlx::query!(
            "INSERT INTO participant_review (uid, vid, point, comment) VALUES (?, ?, ?, ?)",
            uid,
            vid,
            point,
            comment
        ).execute(&self.pool).await?;
        Ok(())
    }
}
