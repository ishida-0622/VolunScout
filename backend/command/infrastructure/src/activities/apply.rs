use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use futures::future;
use sqlx::MySqlPool;

use command_repository::activities::apply::ApplyRepository;
use domain::model::{
    apply::ApplyId,
    user_account::user_id::UserId, volunteer::VolunteerId, group_participants::GroupParticipants, gender::gender_to_i8
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
    async fn create(
        &self,
        aid: ApplyId,
        vid: VolunteerId,
        user_id: UserId,
        as_group: bool,
        members: Option<Vec<GroupParticipants>>
    ) -> Result<()> {
        let aid: String = aid.to_string();
        let vid: String = vid.to_string();
        let uid: String = user_id.to_string();

        sqlx::query!(
            "INSERT INTO apply (aid, vid, uid, applied_at, as_group) VALUES (?, ?, ?, ?, ?)",
            aid,
            vid,
            uid,
            Utc::now(),
            as_group
        ).execute(&self.pool).await?;

        match members {
            Some(mems) => {
                let insert_members_query: Vec<_> = mems
                    .iter()
                    .map(|gp: &GroupParticipants| {
                        sqlx::query!(
                            "INSERT INTO group_participants (gpid, serial, name, furigana, gender, age) VALUES (?, ?, ?, ?, ?, ?)",
                            aid,
                            gp.serial as i16,
                            gp.name.to_string(),
                            gp.furigana.to_string(),
                            gender_to_i8(&gp.gender).unwrap(),
                            gp.age as u8
                        ).execute(&self.pool)
                    })
                    .collect::<Vec<_>>();

                future::try_join_all(
                    insert_members_query
                        .into_iter()
                )
                .await?;
            }
            None => {}
        }

        Ok(())
    }

    async fn update_allowed_status(
        &self,
        aid: ApplyId,
        allowed_status: u8
    ) -> Result<()> {
        let aid: String = aid.to_string();
        sqlx::query!(
            "UPDATE apply SET allowed_status = ?, decided_at = ? WHERE aid = ?",
            allowed_status,
            Utc::now(),
            aid
        )
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn update_is_sent(&self, aid: ApplyId) -> Result<()> {

        let aid: String = aid.to_string();
        sqlx::query!(
            "UPDATE apply SET is_sent = ? WHERE aid = ?",
            true,
            aid
        )
        .execute(&self.pool).await?;
        Ok(())
    }
}
