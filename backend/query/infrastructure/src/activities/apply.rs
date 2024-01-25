use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::{user_account::user_id::UserId, apply::ApplyId, volunteer::VolunteerId};
use query_repository::activities::apply::{Apply, ApplyRepository};

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
}
