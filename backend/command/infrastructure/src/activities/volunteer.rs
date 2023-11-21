use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::MySqlPool;

use command_repository::activities::volunteer::VolunteerRepository;
use domain::model::{
    volunteer::VolunteerId,
    user_account::user_id::UserId
};

pub struct VolunteerImpl {
    pool: MySqlPool,
}

impl VolunteerImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VolunteerRepository for VolunteerImpl {
    async fn create(
        &self,
        vid: VolunteerId,
        gid: UserId,
        title: String,
        message: String,
        overview: String,
        recruited_num: u32,
        place: String,
        start_at: DateTime<Utc>,
        finish_at: DateTime<Utc>,
        deadline_on: NaiveDate,
        as_group: bool,
        terms: Terms
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO volunteer (vid, gid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group, registerd_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            vid.to_string(),
            gid.to_string(),
            title,
            message,
            overview,
            recruited_num,
            place,
            start_at,
            finish_at,
            deadline_on,
            as_group,
            false,
            Utc::now(),
            Utc::now()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(
        &self,
        vid: VolunteerId,
        title: String,
        message: String,
        overview: String,
        recruited_num: u32,
        place: String,
        start_at: DateTime<Utc>,
        finish_at: DateTime<Utc>,
        deadline_on: NaiveDate,
        as_group: bool,
        terms: Terms
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE voluneter SET title = ?, message = ?, overview = ?, recruited_num = ?, place = ?, start_at = ?, finish_at = ?, deadline_on = ?, as_group = ?, updated_at = ?, WHERE vid = ?",
            title,
            message,
            overview,
            recruited_num,
            place,
            start_at,
            finish_at,
            deadline_on,
            as_group,
            Utc::now(),
            vid.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, vid: VolunteerId) -> Result<()> {
        sqlx::query!(
            "UPDATE volunteer SET is_deleted = true, deleted_at = ? WHERE vid = ?",
            Utc::now(),
            vid.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
