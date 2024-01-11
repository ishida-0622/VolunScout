use anyhow::Result;
use async_trait::async_trait;
use domain::consts::conditions::ConditionMap;
use domain::consts::themes::ThemeMap;
use query_repository::MySqlBool;
use sqlx::MySqlPool;

use domain::consts::region::RegionMap;
use domain::model::{user_account::user_id::UserId, volunteer::Volunteer};
use query_repository::user_account::participant::{
    ParticipantAccount, ParticipantCondition, ParticipantRegion, ParticipantTheme,
    ParticipantUserRepository,
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

    async fn find_region_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantRegion>> {
        let response = sqlx::query!(
            r#"
            SELECT rid FROM participant_region WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let region_map = RegionMap::new().regions_index_to_name;

        let regions: Vec<ParticipantRegion> = response
            .into_iter()
            .map(|region| ParticipantRegion {
                name: region_map.get(&(region.rid as usize)).unwrap().to_string(),
            })
            .collect();

        Ok(regions)
    }

    async fn find_theme_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantTheme>> {
        let response = sqlx::query!(
            r#"
            SELECT * FROM participant_element WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let theme_map = ThemeMap::new().themes_id_to_name;

        let themes: Vec<ParticipantTheme> = response
            .into_iter()
            .filter(|element| theme_map.get(&element.eid).is_some())
            .map(|element| ParticipantTheme {
                name: theme_map.get(&element.eid).unwrap().to_string(),
                is_required: MySqlBool::from(element.is_need),
            })
            .collect();

        Ok(themes)
    }

    async fn find_condition_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantCondition>> {
        let response = sqlx::query!(
            r#"
            SELECT * FROM participant_element WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let condition_map = ConditionMap::new().conditions_id_to_name;

        let conditions: Vec<ParticipantCondition> = response
            .into_iter()
            .filter(|element| condition_map.get(&element.eid).is_some())
            .map(|element| ParticipantCondition {
                name: condition_map.get(&element.eid).unwrap().to_string(),
                is_required: MySqlBool::from(element.is_need),
            })
            .collect();

        Ok(conditions)
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

    async fn exists(&self, pid: &UserId) -> Result<bool> {
        let response = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT * FROM participant_account WHERE uid = ?) AS exist
            "#,
            pid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(response.exist == 1)
    }
}
