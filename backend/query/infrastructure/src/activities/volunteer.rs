use anyhow::Result;
use async_trait::async_trait;
use futures::future;
use sqlx::MySqlPool;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use domain::{
    consts::{
        conditions::ConditionMap, region::RegionMap, target_status::TargetStatusMap,
        themes::ThemeMap,
    },
    model::{user_account::user_id::UserId, volunteer::{self, VolunteerId}},
};
use query_repository::activities::volunteer::{
    VolunteerElementsReadModel, VolunteerQueryRepository, VolunteerReadModel,
};

pub struct VolunteerQueryRepositoryImpl {
    pool: MySqlPool,
}

impl VolunteerQueryRepositoryImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VolunteerQueryRepository for VolunteerQueryRepositoryImpl {
    async fn find_elements_by_id(&self, vid: &VolunteerId) -> Result<VolunteerElementsReadModel> {
        let region_query = sqlx::query!(
            r#"
            SELECT rid FROM volunteer_region WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool);

        let element_query = sqlx::query!(
            r#"
            SELECT eid, is_need as "is_need: bool" FROM volunteer_element WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool);

        let (regions, elements) = future::try_join(region_query, element_query).await?;

        let regions_map = RegionMap::new().regions_index_to_name;
        let regions = regions
            .iter()
            .map(|r| regions_map.get(&(r.rid as usize)).unwrap().to_string())
            .collect();

        let themes_map = ThemeMap::new().themes_id_to_name;
        let conditions_map = ConditionMap::new().conditions_id_to_name;
        let target_status_map = TargetStatusMap::new().target_statuses_index_to_name;

        let themes = elements
            .iter()
            .filter_map(|e| {
                if !e.is_need && themes_map.contains_key(&e.eid) {
                    Some(themes_map.get(&e.eid).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let required_themes = elements
            .iter()
            .filter_map(|e| {
                if e.is_need && themes_map.contains_key(&e.eid) {
                    Some(themes_map.get(&e.eid).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let conditions = elements
            .iter()
            .filter_map(|e| {
                if !e.is_need && conditions_map.contains_key(&e.eid) {
                    Some(conditions_map.get(&e.eid).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let required_conditions = elements
            .iter()
            .filter_map(|e| {
                if e.is_need && conditions_map.contains_key(&e.eid) {
                    Some(conditions_map.get(&e.eid).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let target_status = elements
            .iter()
            .filter_map(|e| {
                if target_status_map.contains_key(&e.eid) {
                    Some(target_status_map.get(&e.eid).unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        let volunteer_elements = VolunteerElementsReadModel::new(
            vid.to_string(),
            regions,
            themes,
            required_themes,
            conditions,
            required_conditions,
            target_status,
        );

        Ok(volunteer_elements)
    }

    async fn find_by_id(&self, vid: &VolunteerId) -> Result<VolunteerReadModel> {
        let volunteer = sqlx::query!(
            r#"
            SELECT
                vid, gid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group as "as_group: bool", is_deleted as "is_deleted: bool", deleted_at, registered_at, updated_at
            FROM volunteer WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        if volunteer.is_deleted {return Err(anyhow::anyhow!("the volunteer is deleted"));}

        let elements: VolunteerElementsReadModel = self.find_elements_by_id(&vid).await?;

        let volunteer = VolunteerReadModel::new(
            volunteer.vid,
            volunteer.gid,
            volunteer.title,
            volunteer.message,
            volunteer.overview,
            volunteer.recruited_num as u32,
            volunteer.place,
            volunteer.start_at,
            volunteer.finish_at,
            volunteer.deadline_on,
            volunteer.as_group,
            volunteer.is_deleted,
            if volunteer.deleted_at.is_some() {
                Some(volunteer.deleted_at.unwrap())
            } else {
                None
            },
            volunteer.registered_at,
            volunteer.updated_at,
            elements.regions,
            elements.themes,
            elements.required_themes,
            elements.conditions,
            elements.required_conditions,
            elements.target_status,
        );

        Ok(volunteer)
    }

    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT vid FROM volunteer WHERE gid = ? AND is_deleted = false
            "#,
            gid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;

        Ok(volunteers)
    }

    async fn find_favorite_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT vid FROM favorite WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;
        Ok(volunteers)
    }

    async fn find_activity_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT
            volunteer.vid FROM volunteer, apply WHERE
                volunteer.vid = apply.vid AND
                apply.uid = ? AND
                volunteer.finish_at < now();
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;
        Ok(volunteers)
    }

    async fn find_scheduled_activity_by_id(&self, pid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT
            volunteer.vid FROM volunteer, apply WHERE
                volunteer.vid = apply.vid AND
                apply.uid = ? AND
                volunteer.finish_at > now();
            "#,
            pid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;
        Ok(volunteers)
    }
}
