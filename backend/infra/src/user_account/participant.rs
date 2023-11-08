use anyhow::Result;
use async_trait::async_trait;
use chrono::{NaiveDate, Utc};
use futures::future;
use sqlx::MySqlPool;

use domain::consts::region::RegionMap;
use domain::model::{
    gender::Gender,
    region::Region,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
    volunteer::Volunteer,
};
use domain::repository::user_account::{ParticipantAccount, ParticipantsUserRepository};

pub struct ParticipantsAccountImpl {
    pool: MySqlPool,
}

impl ParticipantsAccountImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ParticipantsUserRepository for ParticipantsAccountImpl {
    async fn create(
        &self,
        pid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: Gender,
        birthday: NaiveDate,
        region: Vec<Region>,
        profile: String,
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

        let futures: Vec<_> = region
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
        future::join_all(futures).await;

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
        region: Vec<Region>,
        profile: String,
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

        let delete_query = sqlx::query!(
            "DELETE FROM participant_region WHERE uid = ?",
            pid.to_string()
        )
        .execute(&self.pool);

        let (update_result, delete_result) = future::join(update_query, delete_query).await;
        update_result?;
        delete_result?;

        let id: String = pid.to_string();
        let futures: Vec<_> = region
            .iter()
            .map(|r| {
                sqlx::query!(
                    "INSERT INTO participant_region (uid, rid) VALUES (?, ?)",
                    id,
                    r.to_uint()
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();
        future::join_all(futures).await;

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
