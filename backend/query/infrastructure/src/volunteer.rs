use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::future;
use sqlx::MySqlPool;

use domain::{
    consts::{
        conditions::ConditionMap, region::RegionMap, target_status::TargetStatusMap,
        themes::ThemeMap,
    },
    model::{user_account::user_id::UserId, volunteer::VolunteerId},
};
use query_repository::volunteer::{VolunteerQueryRepository, VolunteerReadModel};

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
    async fn find_by_id(&self, vid: &VolunteerId) -> Result<VolunteerReadModel> {
        let volunteer_query = sqlx::query!(
            r#"
            SELECT
                vid, gid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group as "as_group: bool", is_deleted as "is_deleted: bool", deleted_at, registered_at, updated_at
            FROM volunteer WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_one(&self.pool);

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

        let (volunteer, regions, elements) =
            future::try_join3(volunteer_query, region_query, element_query).await?;

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

        let volunteer = VolunteerReadModel::new(
            volunteer.vid,
            volunteer.gid,
            volunteer.title,
            volunteer.message,
            volunteer.overview,
            volunteer.recruited_num as u32,
            volunteer.place,
            DateTime::from_naive_utc_and_offset(volunteer.start_at, Utc),
            DateTime::from_naive_utc_and_offset(volunteer.finish_at, Utc),
            volunteer.deadline_on,
            volunteer.as_group,
            volunteer.is_deleted,
            if volunteer.deleted_at.is_some() {
                Some(DateTime::from_naive_utc_and_offset(
                    volunteer.deleted_at.unwrap(),
                    Utc,
                ))
            } else {
                None
            },
            DateTime::from_naive_utc_and_offset(volunteer.registered_at, Utc),
            DateTime::from_naive_utc_and_offset(volunteer.updated_at, Utc),
            regions,
            themes,
            required_themes,
            conditions,
            required_conditions,
            target_status,
        );

        Ok(volunteer)
    }

    async fn find_by_gid(&self, gid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT vid FROM volunteer WHERE gid = ?
            "#,
            gid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;

        Ok(volunteers)
    }
}
