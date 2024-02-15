use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use domain::consts::conditions::ConditionMap;
use domain::consts::target_status::{TargetStatusMap, TARGET_STATUSES_PREFIX};
use domain::consts::themes::ThemeMap;
use domain::model::apply::ApplyId;
use domain::model::condition::Condition;
use domain::model::region::Region;
use domain::model::target_status::TargetStatus;
use domain::model::theme::Theme;
use query_repository::activities::volunteer::VolunteerElementsReadModel;
use sqlx::{query, MySqlPool, Row};

use domain::consts::region::RegionMap;
use domain::model::user_account::user_id::UserId;
use query_repository::user_account::participant::{
    GroupParticipant, ParticipantAccount, ParticipantCondition, ParticipantRegion,
    ParticipantTargetStatus, ParticipantTheme, ParticipantUserRepository, ScoutParticipant,
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
            r#"
            SELECT
                uid, name, furigana, phone, gender, birthday, profile, is_deleted as "is_deleted: bool", deleted_at
            FROM participant_account
            WHERE uid = ?
            "#,
            pid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        if user.is_deleted {
            return Err(anyhow::anyhow!("this participant_account is deleted but existed"));
        }
        Ok(user)
    }

    async fn find_by_ids(&self, pids: &[UserId]) -> Result<Vec<ParticipantAccount>> {
        let params = format!("?{}", ", ?".repeat(pids.len() - 1));
        let query_str = format!(
            r#"
            SELECT *
            FROM participant_account
            WHERE uid IN ({}) AND is_deleted = false
            "#,
            params
        );

        let mut query = sqlx::query(&query_str);
        for pid in pids {
            query = query.bind(pid.to_string());
        }

        let users = query.fetch_all(&self.pool).await?;

        let users = users
            .into_iter()
            .map(|user| ParticipantAccount {
                uid: user.get("uid"),
                name: user.get("name"),
                furigana: user.get("furigana"),
                phone: user.get("phone"),
                gender: user.get("gender"),
                birthday: user.get("birthday"),
                profile: user.get("profile"),
                is_deleted: user.get("is_deleted"),
                deleted_at: user.get("deleted_at"),
            })
            .collect();

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
            SELECT eid, is_need as "is_need: bool"
            FROM participant_element WHERE uid = ?
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
                is_required: element.is_need,
            })
            .collect();

        Ok(themes)
    }

    async fn find_condition_by_id(&self, pid: &UserId) -> Result<Vec<ParticipantCondition>> {
        let response = sqlx::query!(
            r#"
            SELECT eid, is_need as "is_need: bool"
            FROM participant_element
            WHERE uid = ?
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
                is_required: element.is_need,
            })
            .collect();

        Ok(conditions)
    }

    async fn find_target_status_by_id(&self, pid: &UserId) -> Result<ParticipantTargetStatus> {
        let response = sqlx::query!(
            r#"
            SELECT eid FROM participant_element WHERE uid = ? AND eid like ?
            "#,
            pid.to_string(),
            format!("{}%", TARGET_STATUSES_PREFIX)
        )
        .fetch_one(&self.pool)
        .await?;

        let target_status_map = TargetStatusMap::new().target_statuses_index_to_name;

        let target_status = target_status_map.get(&response.eid).unwrap().to_string();

        Ok(ParticipantTargetStatus {
            name: target_status,
        })
    }

    async fn exists(&self, pid: &UserId) -> Result<bool> {
        let response = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT * FROM participant_account WHERE uid = ? AND is_deleted = false) AS exist
            "#,
            pid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(response.exist == 1)
    }

    async fn find_group_participants(&self, aid: &ApplyId) -> Result<Vec<GroupParticipant>> {
        let response = sqlx::query_as!(
            GroupParticipant,
            r#"
            SELECT serial as "serial: u16", name, furigana, gender as "gender: u8", age as "age: u8"
            FROM group_participants
            WHERE gpid = ?
            "#,
            aid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(response)
    }


    /// 参加者の検索
    async fn find_by_elements(
        &self,
        elements: &VolunteerElementsReadModel
    ) -> Result<Vec<ScoutParticipant>> {
        // OR条件の要素一覧
        let mut or_elements: Vec<String> = Vec::new();

        // OR条件の地域一覧
        let mut or_regions = elements
            .regions
            .clone()
            .iter()
            .map(|r: &String| Region::from_str(r).unwrap().to_uint())
            .collect::<Vec<u8>>();

        or_elements.extend(
            elements
                .themes
                .iter()
                .map(|r: &String| Theme::from_str(r).unwrap().to_id()), // .collect::<Vec<String>>()
        );
        or_elements.extend(
            elements
                .conditions
                .iter()
                .map(|r: &String| Condition::from_str(r).unwrap().to_id()), // .collect::<Vec<String>>()
        );
        or_elements.extend(
            elements
                .target_status
                .iter()
                .map(|r: &String| TargetStatus::from_str(r).unwrap().to_id()), // .collect::<Vec<String>>()
        );

        let mut req_elements: Vec<String> = Vec::new();
        req_elements.extend(
            elements
                .required_themes
                .iter()
                .map(|r: &String| Theme::from_str(r).unwrap().to_id()), // .collect::<Vec<String>>()
        );
        req_elements.extend(
            elements
                .required_conditions
                .iter()
                .map(|r: &String| Condition::from_str(r).unwrap().to_id()), // .collect::<Vec<String>>()
        );
        if or_elements.len() == 0 { or_elements.push("".to_string()) }
        if or_regions.len() == 0 { or_regions.push(100 as u8) }
        let query_str = format!(
            r#"
                SELECT
                    participant_account.uid, name, gender, birthday, CAST(AVG(point) AS FLOAT) as point,
                    GROUP_CONCAT(DISTINCT
                        JSON_OBJECT(
                            'eid', participant_element.eid,
                            'is_need', participant_element.is_need
                        )
                    ) as eids,
                    GROUP_CONCAT(DISTINCT
                        JSON_OBJECT(
                            'rid', participant_region.rid
                        )
                    ) as rids,
                    COUNT(DISTINCT CASE WHEN participant_element.eid IN ({}) THEN participant_element.eid END) AS eid_match_count,
                    COUNT(DISTINCT CASE WHEN participant_region.rid IN ({}) THEN participant_region.rid END) AS rid_match_count,
                    COUNT(DISTINCT CASE WHEN participant_element.is_need = true AND participant_element.eid IN ({}) THEN participant_element.eid END) AS req_eid_match_count
                FROM
                    participant_account
                LEFT JOIN
                    participant_element ON participant_account.uid = participant_element.uid
                LEFT JOIN
                    participant_region ON participant_account.uid = participant_region.uid
                LEFT JOIN
                    participant_review ON participant_account.uid = participant_review.uid
                WHERE
                    participant_account.uid IN (
                        SELECT participant_account.uid
                        FROM participant_account
                        LEFT JOIN participant_element ON participant_account.uid = participant_element.uid
                        LEFT JOIN participant_region ON participant_account.uid = participant_region.uid
                        {}
                        GROUP BY participant_account.uid
                        {}
                    )
                AND NOT participant_account.is_deleted
                GROUP BY
                    participant_account.uid
                ORDER BY
                    eid_match_count + rid_match_count DESC, req_eid_match_count DESC, point DESC, uid DESC;
            "#,
            format!("?{}", ", ?".repeat(or_elements.len() - 1)),
            format!("?{}", ", ?".repeat(or_regions.len() - 1)),
            format!("?{}", ", ?".repeat(or_elements.len() + req_elements.len() - 1)),

            if req_elements.len() > 0 {
                format!(
                    "WHERE participant_element.eid IN ({})",
                    format!("?{}", ", ?".repeat(req_elements.len() - 1))
                )
            } else {"".to_string()},

            if req_elements.len() > 0 {
                format!(
                    "HAVING COUNT(DISTINCT participant_element.eid) = {}",
                    req_elements.len()
                )
            } else {"".to_string()}
        );

        let mut query = query(&query_str);

        for or_element in or_elements.clone() {
            query = query.bind(or_element);
        }
        for or_region in or_regions {
            query = query.bind(or_region);
        }

        for or_element in or_elements {
            query = query.bind(or_element.to_string());
        }
        for req_element in req_elements.clone() {
            query = query.bind(req_element.to_string());
        }

        for req_element in req_elements.clone() {
            query = query.bind(req_element.to_string());
        }

        let participants = query.fetch_all(&self.pool).await?;

        let participants = participants
            .into_iter()
            .map(|participant| {
                ScoutParticipant {
                    uid: participant.get("uid"),
                    name: participant.get("name"),
                    gender: participant.get::<i8, _>("gender") as i8,
                    birthday: participant.get("birthday"),
                    point: participant.get::<Option<f32>, _>("point")
                }
            })
            .collect();

        Ok(participants)
    }

}
