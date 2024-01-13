use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use futures::future;
use sqlx::MySqlPool;

use command_repository::user_account::participant::ParticipantUserRepository;
use domain::model::{
    condition::Condition,
    gender::Gender,
    region::Region,
    theme::Theme,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    }, terms::Terms, target_status::TargetStatus
};

pub struct ParticipantAccountImpl {
    pool: MySqlPool,
}

impl ParticipantAccountImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ParticipantUserRepository for ParticipantAccountImpl {
    async fn create(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        profile: String,
        terms: Terms
    ) -> Result<()> {
        let id: String = pid.to_string();

        sqlx::query!("INSERT INTO participant_account (uid, name, furigana, gender, birthday, phone, profile) VALUES (?, ?, ?, ?, ?, ?, ?)",
            id,
            name.to_string(),
            furigana.to_string(),
            gender as u8,
            birthday,
            phone.to_string(),
            profile
        ).execute(&self.pool).await?;

        let insert_region_query: Vec<_> = terms.regions
            .iter()
            .map(|r: &Region| {
                sqlx::query!(
                    "INSERT INTO participant_region (uid, rid) VALUES (?, ?)",
                    id,
                    r.to_uint()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_query: Vec<_> = terms.themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_required_query: Vec<_> = terms.required_themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    t.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_query: Vec<_> = terms.conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    c.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_required_query: Vec<_> = terms.required_conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    c.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_target_status_query: Vec<_> = terms.target_status
            .iter()
            .map(|t: &TargetStatus| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(
            insert_region_query
                .into_iter()
                .chain(insert_theme_query)
                .chain(insert_theme_required_query)
                .chain(insert_condition_query)
                .chain(insert_condition_required_query)
                .chain(insert_target_status_query),
        )
        .await?;

        Ok(())
    }

    async fn update(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        profile: String,
        terms: Terms
    ) -> Result<()> {
        let update_query = sqlx::query!(
            "UPDATE participant_account SET name = ?,furigana = ?, phone = ?, gender = ?, birthday = ?, profile = ? WHERE uid = ?",
            name.to_string(),
            furigana.to_string(),
            phone.to_string(),
            gender as u8,
            birthday,
            profile,
            pid.to_string()
        )
        .execute(&self.pool);

        let delete_region_query = sqlx::query!(
            "DELETE FROM participant_region WHERE uid = ?",
            pid.to_string()
        )
        .execute(&self.pool);

        let delete_element_query = sqlx::query!(
            "DELETE FROM participant_element WHERE uid = ?",
            pid.to_string()
        )
        .execute(&self.pool);

        future::try_join3(update_query, delete_region_query, delete_element_query).await?;

        let id: String = pid.to_string();

        let insert_region_query: Vec<_> = terms.regions
            .iter()
            .map(|r: &Region| {
                sqlx::query!(
                    "INSERT INTO participant_region (uid, rid) VALUES (?, ?)",
                    id,
                    r.to_uint()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_query: Vec<_> = terms.themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_required_query: Vec<_> = terms.required_themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    t.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_query: Vec<_> = terms.conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    c.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_required_query: Vec<_> = terms.required_conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    c.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_target_status_query: Vec<_> = terms.target_status
            .iter()
            .map(|t: &TargetStatus| {
                sqlx::query!(
                    "INSERT INTO participant_element (uid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(
            insert_region_query
                .into_iter()
                .chain(insert_theme_query)
                .chain(insert_theme_required_query)
                .chain(insert_condition_query)
                .chain(insert_condition_required_query)
                .chain(insert_target_status_query),
        )
        .await?;

        Ok(())
    }

    async fn delete(&self, pid: UserId) -> Result<()> {
        sqlx::query!(
            "UPDATE participant_account SET is_deleted = true, deleted_at = ? WHERE uid = ?",
            Utc::now(),
            pid.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
