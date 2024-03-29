use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::{apply::ApplyId, user_account::user_id::UserId, volunteer::VolunteerId};
use query_repository::activities::apply::{
    Apply, ApplyRepository, PastVolunteerParticipantReadModel,
};

pub struct ApplyImpl {
    pool: MySqlPool,
}

impl ApplyImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ApplyRepository for ApplyImpl {
    async fn find_by_sid(&self, aid: &ApplyId) -> Result<Apply> {
        let apply: Apply = sqlx::query_as!(
            Apply,
            r#"
            SELECT
                aid, vid, uid, applied_at, as_group as "as_group: bool", allowed_status, decided_at, is_sent as "is_sent: bool"
            FROM apply
            WHERE aid = ?
            "#,
            aid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(apply)
    }

    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<Apply>> {
        let apply = sqlx::query_as!(
            Apply,
            r#"
            SELECT
                aid, vid, uid, applied_at, as_group as "as_group: bool", allowed_status, decided_at, is_sent as "is_sent: bool"
            FROM apply
            WHERE vid IN
                (select vid from volunteer where gid = ?)
            "#,
            gid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(apply)
    }

    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Apply>> {
        let apply = sqlx::query_as!(
            Apply,
            r#"
            SELECT
                aid, vid, uid, applied_at, as_group as "as_group: bool", allowed_status, decided_at, is_sent as "is_sent: bool"
            FROM apply
            WHERE uid = ?
            "#,
            uid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(apply)
    }

    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Apply>> {
        let apply = sqlx::query_as!(
            Apply,
            r#"
            SELECT
                aid, vid, uid, applied_at, as_group as "as_group: bool", allowed_status, decided_at, is_sent as "is_sent: bool"
            FROM apply
            WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(apply)
    }

    async fn find_past_volunteer_participants(
        &self,
        vid: &VolunteerId,
    ) -> Result<Vec<PastVolunteerParticipantReadModel>> {
        let participants = sqlx::query_as!(
            PastVolunteerParticipantReadModel,
            r#"
            SELECT uid, name, gender as "gender: u8", birthday
            FROM participant_account
            WHERE uid IN (
                SELECT uid
                FROM apply
                WHERE vid = ? AND allowed_status = 1
            )
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(participants)
    }

    async fn exists_apply(&self, vid: &VolunteerId, uid: &UserId) -> Result<bool> {
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM apply
                WHERE vid = ? AND uid = ?
            ) as "exists: bool"
            "#,
            vid.to_string(),
            uid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.exists)
    }
}
