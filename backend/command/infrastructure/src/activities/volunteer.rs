use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use futures::future;
use sqlx::MySqlPool;

use command_repository::activities::volunteer::VolunteerRepository;
use domain::model::{
    condition::Condition, region::Region, target_status::TargetStatus, terms::Terms, theme::Theme, user_account::user_id::UserId, volunteer::VolunteerId
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
        reward: Option<String>,
        terms: Terms,
        s3_keys: Vec<String>
    ) -> Result<()> {
        let id: String = vid.to_string();

        sqlx::query!(
            "INSERT INTO volunteer (vid, gid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group, reward, registered_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            id,
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
            reward,
            Utc::now(),
            Utc::now()
        ).execute(&self.pool).await?;

        let insert_target_status_query: Vec<_> = terms
            .target_status
            .iter()
            .map(|t: &TargetStatus| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_region_query: Vec<_> = terms
            .regions
            .iter()
            .map(|r: &Region| {
                sqlx::query!(
                    "INSERT INTO volunteer_region (vid, rid) VALUES (?, ?)",
                    id,
                    r.to_uint()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_query: Vec<_> = terms
            .themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_required_query: Vec<_> = terms
            .required_themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    t.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_query: Vec<_> = terms
            .conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    c.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_required_query: Vec<_> = terms
            .required_conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    c.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_photo_query: Vec<_> = s3_keys
            .iter()
            .map(|p: &String| {
                sqlx::query!(
                    "INSERT INTO volunteer_photo VALUES (?, ?)",
                    p,
                    id
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(
            insert_target_status_query
                .into_iter()
                .chain(insert_region_query)
                .chain(insert_theme_query)
                .chain(insert_theme_required_query)
                .chain(insert_condition_query)
                .chain(insert_condition_required_query)
                .chain(insert_photo_query),
        )
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
        reward: Option<String>,
        terms: Terms,
        s3_keys: Vec<String>
    ) -> Result<()> {
        let id: String = vid.to_string();

        let update_query = sqlx::query!(
            "UPDATE volunteer SET title = ?, message = ?, overview = ?, recruited_num = ?, place = ?, start_at = ?, finish_at = ?, deadline_on = ?, as_group = ?, reward = ?, updated_at = ? WHERE vid = ?",
            title,
            message,
            overview,
            recruited_num,
            place,
            start_at,
            finish_at,
            deadline_on,
            as_group,
            reward,
            Utc::now(),
            id
        )
        .execute(&self.pool);

        let delete_region_query =
            sqlx::query!("DELETE FROM volunteer_region WHERE vid = ?", id).execute(&self.pool);

        let delete_element_query =
            sqlx::query!("DELETE FROM volunteer_element WHERE vid = ?", id).execute(&self.pool);

        let delete_photo_query =
            sqlx::query!("DELETE FROM volunteer_photo WHERE vid = ?", id).execute(&self.pool);

        future::try_join4(update_query, delete_region_query, delete_element_query, delete_photo_query).await?;

        let insert_target_status_query: Vec<_> = terms
            .target_status
            .iter()
            .map(|t: &TargetStatus| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_region_query: Vec<_> = terms
            .regions
            .iter()
            .map(|r: &Region| {
                sqlx::query!(
                    "INSERT INTO volunteer_region (vid, rid) VALUES (?, ?)",
                    id,
                    r.to_uint()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_query: Vec<_> = terms
            .themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    t.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_theme_required_query: Vec<_> = terms
            .required_themes
            .iter()
            .map(|t: &Theme| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    t.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_query: Vec<_> = terms
            .conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid) VALUES (?, ?)",
                    id,
                    c.to_id()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_condition_required_query: Vec<_> = terms
            .required_conditions
            .iter()
            .map(|c: &Condition| {
                sqlx::query!(
                    "INSERT INTO volunteer_element (vid, eid, is_need) VALUES (?, ?, ?)",
                    id,
                    c.to_id(),
                    true
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        let insert_photo_query: Vec<_> = s3_keys
            .iter()
            .map(|p: &String| {
                sqlx::query!(
                    "INSERT INTO volunteer_photo VALUES (?, ?)",
                    p,
                    id
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(
            insert_target_status_query
                .into_iter()
                .chain(insert_region_query)
                .chain(insert_theme_query)
                .chain(insert_theme_required_query)
                .chain(insert_condition_query)
                .chain(insert_condition_required_query)
                .chain(insert_photo_query),
        )
        .await?;
        Ok(())
    }

    async fn delete(&self, vid: VolunteerId) -> Result<()> {
        let id: String = vid.to_string();
        struct IsExists {
            is_deleted: bool
        }

        let is_deleted = sqlx::query_as!(
            IsExists,
            r#"
            SELECT is_deleted as "is_deleted: bool" FROM volunteer WHERE vid = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        if is_deleted.is_deleted {
            Err(anyhow::anyhow!("This volunteer is already deleted".to_string()))
        } else {
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

    async fn register_favorite(&self, uid: UserId, vid: VolunteerId) -> Result<()> {
        sqlx::query!(
            "INSERT INTO favorite VALUES(?, ?, ?)",
            uid.to_string(),
            vid.to_string(),
            Utc::now()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn unregister_favorite(&self, uid: UserId, vid: VolunteerId) -> Result<()> {
        sqlx::query!(
            "DELETE FROM favorite where uid = ? AND vid = ?",
            uid.to_string(),
            vid.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
