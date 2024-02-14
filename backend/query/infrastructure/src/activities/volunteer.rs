use std::str::FromStr;

use anyhow::Result;
use async_trait::async_trait;
use futures::future;
use serde_json::Value;
use sqlx::{query, MySqlPool, Row};
use domain::{
    consts::{
        conditions::ConditionMap, region::RegionMap, target_status::TargetStatusMap,
        themes::ThemeMap,
    },
    model::{
        condition::Condition, region::Region, target_status::TargetStatus, theme::Theme,
        user_account::user_id::UserId, volunteer::VolunteerId,
    },
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
    /// vidで一致するボランティア要素の取得
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
            None,
            themes,
            required_themes,
            conditions,
            required_conditions,
            target_status,
        );

        Ok(volunteer_elements)
    }

    ///vidで一致するボランティア情報の取得
    async fn find_by_id(&self, vid: &VolunteerId) -> Result<VolunteerReadModel> {
        let volunteer = sqlx::query!(
            r#"
            SELECT
                vid, gid, title, message, overview, recruited_num, place, reward, start_at, finish_at, deadline_on, as_group as "as_group: bool", is_deleted as "is_deleted: bool", deleted_at, registered_at, updated_at
            FROM volunteer WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        if volunteer.is_deleted {
            return Err(anyhow::anyhow!("the volunteer is deleted"));
        }

        let elements: VolunteerElementsReadModel = self.find_elements_by_id(&vid).await?;
        let photos = sqlx::query!(
            r#"
            SELECT s3_key FROM volunteer_photo WHERE vid = ?
            "#,
            vid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let volunteer = VolunteerReadModel::new(
            volunteer.vid,
            volunteer.gid,
            volunteer.title,
            volunteer.message,
            volunteer.overview,
            volunteer.recruited_num as u32,
            volunteer.place,
            volunteer.reward,
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
            photos.into_iter().map(|p| p.s3_key).collect(),
        );

        Ok(volunteer)
    }

    ///ボランティアの検索
    async fn find_by_elements(
        &self,
        elements: &VolunteerElementsReadModel,
        search_words: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        // OR条件の要素一覧
        let mut or_elements: Vec<String> = Vec::new();

        // OR条件の地域一覧
        let or_regions = elements
            .regions
            .clone()
            .iter()
            .map(|r: &String| Region::from_str(r).unwrap().to_uint())
            .collect::<Vec<u8>>();
        let req_regions: Vec<u8> = match elements.required_regions.clone() {
            Some(regions) => regions
                .clone()
                .iter()
                .map(|r: &String| Region::from_str(r).unwrap().to_uint())
                .collect::<Vec<u8>>(),
            None => Vec::new(),
        };
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
                    volunteer.vid,
                    volunteer.gid, title, message, overview, recruited_num, place, start_at, finish_at, as_group, reward, volunteer.is_deleted,
                    volunteer.deleted_at, deadline_on, registered_at, updated_at, is_paid, GROUP_CONCAT(DISTINCT volunteer_photo.s3_key) AS s3_keys,
                    GROUP_CONCAT(DISTINCT
                        JSON_OBJECT(
                            'eid', volunteer_element.eid,
                            'is_need', volunteer_element.is_need
                        )
                    ) as eids,
                    GROUP_CONCAT(DISTINCT
                            volunteer_region.rid
                    ) as rids,
                    COUNT(DISTINCT CASE WHEN volunteer_element.eid IN ({}) THEN volunteer_element.eid END) AS eid_match_count,
                    COUNT(DISTINCT CASE WHEN volunteer_region.rid IN ({}) THEN volunteer_region.rid END) AS rid_match_count,
                    COUNT(DISTINCT CASE WHEN volunteer_element.is_need = true AND volunteer_element.eid IN ({}) THEN volunteer_element.eid END) AS req_eid_match_count
                FROM
                    volunteer
                LEFT JOIN
                    volunteer_element ON volunteer.vid = volunteer_element.vid
                LEFT JOIN
                    volunteer_region ON volunteer.vid = volunteer_region.vid
                LEFT JOIN
                    group_account ON volunteer.gid = group_account.gid
                LEFT JOIN
                    volunteer_photo ON volunteer.vid = volunteer_photo.vid
                WHERE
                    volunteer.vid IN (
                        SELECT volunteer.vid
                        FROM volunteer
                        LEFT JOIN volunteer_element ON volunteer.vid = volunteer_element.vid
                        LEFT JOIN volunteer_region ON volunteer.vid = volunteer_region.vid
                        {}
                        GROUP BY volunteer.vid
                        {}
                    )
                AND NOT volunteer.is_deleted
                AND deadline_on > NOW()
                AND title LIKE "%{}%"
                GROUP BY
                    volunteer.vid
                ORDER BY
                    eid_match_count + rid_match_count DESC, is_paid DESC, req_eid_match_count DESC, deadline_on ASC, registered_at DESC, vid;
            "#,
            format!("?{}", ", ?".repeat(or_elements.len() - 1)),
            format!("?{}", ", ?".repeat(or_regions.len() - 1)),
            format!("?{}", ", ?".repeat(or_elements.len() + req_elements.len() - 1)),

            if req_elements.len() > 0 && req_regions.len() > 0 {
                format!(
                    "WHERE volunteer_element.eid IN ({}) AND volunteer_region.rid IN ({})",
                    format!("?{}", ", ?".repeat(req_elements.len() - 1)),
                    format!("?{}", ", ?".repeat(req_regions.len() - 1))
                )
            } else if req_elements.len() > 0 {
                format!(
                    "WHERE volunteer_element.eid IN ({})",
                    format!("?{}", ", ?".repeat(req_elements.len() - 1))
                )
            } else if req_regions.len() > 0 {
                format!(
                    "WHERE volunteer_region.rid IN ({})",
                    format!("?{}", ", ?".repeat(req_regions.len() - 1))
                )
            } else {"".to_string()},

            if req_elements.len() > 0 && req_regions.len() > 0 {
                format!(
                    "HAVING COUNT(DISTINCT volunteer_element.eid) = {} AND COUNT(DISTINCT volunteer_region.rid) = {}",
                    req_elements.len(),
                    req_regions.len()
                )
            } else if req_elements.len() > 0 {
                format!(
                    "HAVING COUNT(DISTINCT volunteer_element.eid) = {}",
                    req_elements.len()
                )
            } else if req_regions.len() > 0 {
                format!(
                    "HAVING COUNT(DISTINCT volunteer_region.rid) = {}",
                    req_regions.len()
                )
            } else {"".to_string()},
            search_words
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
        for req_region in req_regions.clone() {
            query = query.bind(req_region);
        }

        let volunteers = query.fetch_all(&self.pool).await?;

        let regions_map = RegionMap::new().regions_index_to_name;
        let themes_map = ThemeMap::new().themes_id_to_name;
        let conditions_map = ConditionMap::new().conditions_id_to_name;
        let target_status_map = TargetStatusMap::new().target_statuses_index_to_name;

        let volunteers = volunteers
            .into_iter()
            .map(|volunteer| {
                // println!("Debug info: {:?}", volunteer.get::<String, _>("eids"));
                let elements: Vec<Value> = volunteer
                    .get::<String, _>("eids")
                    .split("},{")
                    .map(|s| {
                        // 文字列をJSONオブジェクトにパースする
                        let new_s = {
                            let first_char = s.chars().next().unwrap();
                            let last_char = s.chars().last().unwrap();
                            if last_char != '}' && first_char != '{' {
                                format!("{}{}{}", "{", s, "}")
                            } else if last_char != '}' {
                                format!("{}{}", s, "}")
                            } else if first_char != '{' {
                                format!("{}{}", "{", s)
                            } else {
                                s.to_owned()
                            }
                        };
                        serde_json::from_str(&new_s)
                            .unwrap_or_else(|_| panic!("Failed to parse JSON: {}", s))
                    })
                    .collect();

                let themes: Vec<String> = elements
                    .iter()
                    .filter_map(|e| {
                        // オブジェクトからeidとis_needを取得する
                        let eid = e.get("eid").and_then(|v| v.as_str()).unwrap();
                        let is_need = e.get("is_need").and_then(|v| v.as_u64()).unwrap() == 1;

                        // themes_mapにeidが含まれており、かつis_needがfalseであればテーマを取得する
                        if let Some(theme) = themes_map.get(eid) {
                            if !is_need {
                                Some(theme.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let required_themes: Vec<String> = elements
                    .iter()
                    .filter_map(|e| {
                        // オブジェクトからeidとis_needを取得する
                        let eid = e.get("eid").and_then(|v| v.as_str()).unwrap();
                        let is_need = e.get("is_need").and_then(|v| v.as_u64()).unwrap() == 1;

                        // themes_mapにeidが含まれており、かつis_needがtrueであればテーマを取得する
                        if let Some(theme) = themes_map.get(eid) {
                            if is_need {
                                Some(theme.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let conditions: Vec<String> = elements
                    .iter()
                    .filter_map(|e| {
                        // オブジェクトからeidとis_needを取得する
                        let eid = e.get("eid").and_then(|v| v.as_str()).unwrap();
                        let is_need = e.get("is_need").and_then(|v| v.as_u64()).unwrap() == 1;

                        // conditions_mapにeidが含まれており、かつis_needがfalseであればテーマを取得する
                        if let Some(condition) = conditions_map.get(eid) {
                            if !is_need {
                                Some(condition.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let required_conditions: Vec<String> = elements
                    .iter()
                    .filter_map(|e| {
                        // オブジェクトからeidとis_needを取得する
                        let eid = e.get("eid").and_then(|v| v.as_str()).unwrap();
                        let is_need = e.get("is_need").and_then(|v| v.as_u64()).unwrap() == 1;

                        // conditions_mapにeidが含まれており、かつis_needがtrueであればテーマを取得する
                        if let Some(condition) = conditions_map.get(eid) {
                            if is_need {
                                Some(condition.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let target_statuses: Vec<String> = elements
                    .iter()
                    .filter_map(|e| {
                        // オブジェクトからeidとis_needを取得する
                        let eid = e.get("eid").and_then(|v| v.as_str()).unwrap();
                        let is_need = e.get("is_need").and_then(|v| v.as_u64()).unwrap() == 1;

                        // target_statuss_mapにeidが含まれており、かつis_needがfalseであればテーマを取得する
                        if let Some(target_status) = target_status_map.get(eid) {
                            if !is_need {
                                Some(target_status.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect();

                let regions: Vec<String> = volunteer
                    .get::<String, _>("rids")
                    .split(',')
                    .map(|rid| {
                        let rid = rid.trim();
                        regions_map
                            .get(&rid.parse::<usize>().unwrap())
                            .unwrap()
                            .to_string()
                    })
                    .collect();

                let photo_urls: Vec<String> = match volunteer.get::<Option<String>, _>("s3_keys") {
                    Some(keys) => {
                        keys
                        .split(',')
                        .map(|key: &str| {
                            println!("Debug info: {:?}", key);
                            key.to_string()
                        })
                        .collect()
                    }
                    None => Vec::new()
                };

                VolunteerReadModel {
                    vid: volunteer.get("vid"),
                    gid: volunteer.get("gid"),
                    title: volunteer.get("title"),
                    message: volunteer.get("message"),
                    overview: volunteer.get("overview"),
                    recruited_num: volunteer.get::<u32, _>("recruited_num"),
                    place: volunteer.get("place"),
                    reward: volunteer.get("reward"),
                    start_at: volunteer.get("start_at"),
                    finish_at: volunteer.get("finish_at"),
                    deadline_on: volunteer.get("deadline_on"),
                    as_group: volunteer.get("as_group"),
                    is_deleted: volunteer.get("is_deleted"),
                    deleted_at: volunteer.get("deleted_at"),
                    registered_at: volunteer.get("registered_at"),
                    updated_at: volunteer.get("updated_at"),
                    photo_urls: Vec::new(),
                    themes: themes,
                    regions: regions,
                    conditions: conditions,
                    required_themes: required_themes,
                    required_conditions: required_conditions,
                    target_status: target_statuses,
                }
            })
            .collect();

        Ok(volunteers)
    }

    ///gidが一致する団体が登録したボランティア情報の取得
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

    ///uidが一致する参加者のお気に入りボランティア情報取得
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

    ///uidが一致する参加者の活動履歴ボランティア情報の取得
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

    ///uidが一致する参加者の活動予定ボランティア情報の取得
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

    async fn find_activity_by_gid(&self, gid: &UserId) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT vid FROM volunteer WHERE gid = ? AND finish_at < now() AND is_deleted = false
            "#,
            gid.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let vids: Vec<VolunteerId> = vids.iter().map(|v| VolunteerId::from_str(&v.vid)).collect();

        let volunteers = future::try_join_all(vids.iter().map(|vid| self.find_by_id(&vid))).await?;
        Ok(volunteers)
    }

    async fn find_scheduled_activity_by_gid(
        &self,
        gid: &UserId,
    ) -> Result<Vec<VolunteerReadModel>> {
        let vids = sqlx::query!(
            r#"
            SELECT vid FROM volunteer WHERE gid = ? AND finish_at > now() AND is_deleted = false
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
