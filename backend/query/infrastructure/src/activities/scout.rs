use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::{user_account::user_id::UserId, scout::ScoutId, volunteer::VolunteerId};
use query_repository::activities::scout::{Scout, ScoutRepository};

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
    async fn find_by_sid(&self, sid: &ScoutId) -> Result<Scout> {
        // sqlx::query!("SET log_statements = ON")
        // .execute(&self.pool)
        // .await?;

        let scout: Scout = sqlx::query_as!(
            Scout,
            r#"
            SELECT
                sid, vid, uid, message, scouted_at, is_read as "is_read: bool", is_sent as "is_sent: bool", sent_at, is_denied as "is_denied: bool", denied_at
            FROM scout
            WHERE sid = ?
            "#,
            sid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
    // sqlx::query!("SET log_statements = OFF")?
    //     .execute(&self.pool)
    //     .await?;

        Ok(scout)
    }

    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<Scout>> {
        let scout = sqlx::query_as!(
            Scout,
            r#"
            SELECT
                sid, vid, uid, message, scouted_at, is_read as "is_read: bool", is_sent as "is_sent: bool", sent_at, is_denied as "is_denied: bool", denied_at
            FROM scout
            WHERE vid IN
                (select vid from volunteer where gid = ?)
            "#,
            gid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scout)
    }

    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Scout>> {
        let scout = sqlx::query_as!(
            Scout,
            r#"
            SELECT
                sid, vid, uid, message, scouted_at, is_read as "is_read: bool", is_sent as "is_sent: bool", sent_at, is_denied as "is_denied: bool", denied_at
            FROM scout
            WHERE uid = ?
            "#,
            uid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scout)
    }

    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Scout>> {
        let scout = sqlx::query_as!(
            Scout,
            r#"
            SELECT
                sid, vid, uid, message, scouted_at, is_read as "is_read: bool", is_sent as "is_sent: bool", sent_at, is_denied as "is_denied: bool", denied_at
            FROM scout
            WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scout)
    }

    async fn find_all(&self) -> Result<Vec<Scout>> {
        let scout = sqlx::query_as!(
            Scout,
            r#"
            SELECT
                sid, vid, uid, message, scouted_at, is_read as "is_read: bool", is_sent as "is_sent: bool", sent_at, is_denied as "is_denied: bool", denied_at
            FROM scout
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scout)
    }
}
