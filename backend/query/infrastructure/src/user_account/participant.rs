use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::consts::region::RegionMap;
use domain::model::{region::Region, user_account::user_id::UserId, volunteer::Volunteer};
use query_repository::user_account::participant::{ParticipantAccount, ParticipantUserRepository};

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
    async fn find_by_id(&self, pid: &UserId) -> Result<ParticipantAccount> {
        let user = sqlx::query_as!(
            ParticipantAccount,
            "SELECT * FROM participant_account WHERE uid = ?",
            pid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn find_by_ids(&self, pids: &[UserId]) -> Result<Vec<ParticipantAccount>> {
        let users = sqlx::query_as!(
            ParticipantAccount,
            r#"
            SELECT * FROM participant_account
            WHERE uid IN (?)
            "#,
            pids.iter()
                .map(|pid| pid.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    async fn find_region_by_id(&self, pid: &UserId) -> Result<Vec<Region>> {
        let response = sqlx::query!(
            r#"
            SELECT rid FROM participant_region WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let region_map = RegionMap::new().regions_index_to_name;

        let regions: Vec<Region> = response
            .into_iter()
            .map(|region| {
                Region::new(region_map.get(&(region.rid as usize)).unwrap().to_string()).unwrap()
            })
            .collect::<Vec<Region>>();

        Ok(regions)
    }

    async fn find_favorite_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>> {
        todo!()
    }

    async fn find_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>> {
        todo!()
    }

    async fn find_scheduled_activity_by_id(&self, pid: &UserId) -> Result<Vec<Volunteer>> {
        todo!()
    }
}
