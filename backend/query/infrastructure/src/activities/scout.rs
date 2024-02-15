use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::{scout::ScoutId, user_account::user_id::UserId, volunteer::VolunteerId};
use query_repository::activities::scout::{Scout, ScoutFromGroup, ScoutRepository};

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
            AND is_denied = false
            "#,
            uid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(scout)
    }

    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<ScoutFromGroup>> {
        let scout = sqlx::query!(
            r#"
            SELECT
                s.sid, s.vid, s.uid, s.message, s.scouted_at, s.is_read as "is_read: bool", s.is_sent as "is_sent: bool", s.sent_at, s.is_denied as "is_denied: bool", s.denied_at, p.name, p.gender as "gender: u8", p.birthday, AVG(r.point) as point
            FROM scout as s
            INNER JOIN participant_account as p ON s.uid = p.uid
            LEFT JOIN participant_review as r ON p.uid = r.uid
            WHERE s.vid = ?
            GROUP BY s.sid
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        let scout = scout
            .into_iter()
            .map(|s| ScoutFromGroup {
                sid: s.sid,
                vid: s.vid,
                uid: s.uid,
                message: s.message,
                scouted_at: s.scouted_at,
                is_read: s.is_read,
                is_sent: s.is_sent,
                sent_at: s.sent_at,
                is_denied: s.is_denied,
                denied_at: s.denied_at,
                name: s.name,
                gender: s.gender,
                birthday: s.birthday,
                point: match s.point {
                    Some(p) => {
                        Some((p.to_string().parse::<f32>().unwrap() * 100.0).round() / 100.0)
                    }
                    None => None,
                },
            })
            .collect();
        Ok(scout)
    }
}
