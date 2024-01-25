use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::{user_account::user_id::UserId, volunteer::VolunteerId};
use query_repository::activities::review::{Review, ParticipantReviewRepository, VolunteerReviewRepository};

pub struct ReviewImpl {
    pool: MySqlPool,
}

impl ReviewImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ParticipantReviewRepository for ReviewImpl {
    async fn find_by_ids(&self, uid: &UserId, vid: &VolunteerId) -> Result<Review> {
        let review: Review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM participant_review WHERE uid = ? and vid = ?
            "#,
            uid.to_string(),
            vid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(review)
    }

    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Review>> {
        let review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM participant_review WHERE uid = ?
            "#,
            uid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(review)
    }

    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Review>> {
        let review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM participant_review WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(review)
    }
}

#[async_trait]
impl VolunteerReviewRepository for ReviewImpl {
    async fn find_by_ids(&self, uid: &UserId, vid: &VolunteerId) -> Result<Review> {
        let review: Review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM volunteer_review WHERE uid = ? and vid = ?
            "#,
            uid.to_string(),
            vid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(review)
    }

    async fn find_by_uid(&self, uid: &UserId) -> Result<Vec<Review>> {
        let review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM volunteer_review WHERE uid = ?
            "#,
            uid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(review)
    }

    async fn find_by_vid(&self, vid: &VolunteerId) -> Result<Vec<Review>> {
        let review = sqlx::query_as!(
            Review,
            r#"
            SELECT * FROM volunteer_review WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(review)
    }
}
