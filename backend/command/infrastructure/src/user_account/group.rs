use anyhow::Result;
use async_trait::async_trait;
use futures::future;
use chrono::Utc;
use sqlx::MySqlPool;

use command_repository::user_account::group::GroupUserRepository;
use domain::model::user_account::{
    user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
    user_phone::UserPhone,
};

pub struct GroupAccountImpl {
    pool: MySqlPool,
}

impl GroupAccountImpl {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GroupUserRepository for GroupAccountImpl {
    async fn create(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        representative_name: UserName,
        representative_furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
        s3_keys: Vec<String>
    ) -> Result<()> {
        let id: String = gid.to_string();
        sqlx::query!(
            "INSERT INTO group_account (gid, name, furigana, representative_name, representative_furigana, phone, address, contents) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            id,
            name.to_string(),
            furigana.to_string(),
            representative_name.to_string(),
            representative_furigana.to_string(),
            phone.to_string(),
            address,
            contents
        )
        .execute(&self.pool)
        .await?;

        let insert_photo_query: Vec<_> = s3_keys
            .iter()
            .map(|p: &String| {
                sqlx::query!(
                    "INSERT INTO group_photo VALUES (?, ?)",
                    p,
                    id
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(insert_photo_query.into_iter()).await?;

        Ok(())
    }

    async fn update(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        representative_name: UserName,
        representative_furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
        s3_keys: Vec<String>
    ) -> Result<()> {
        let id: String = gid.to_string();
        sqlx::query!(
            "UPDATE group_account SET name = ?, furigana = ?, representative_name = ?, representative_furigana = ?, phone = ?, address = ?, contents = ? WHERE gid = ?",
            name.to_string(),
            furigana.to_string(),
            representative_name.to_string(),
            representative_furigana.to_string(),
            phone.to_string(),
            address,
            contents,
            id
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            "DELETE FROM group_photo WHERE gid = ?",
            id
        )
        .execute(&self.pool)
        .await?;

        let insert_photo_query: Vec<_> = s3_keys
            .iter()
            .map(|p: &String| {
                sqlx::query!(
                    "INSERT INTO group_photo VALUES (?, ?)",
                    p,
                    id
                )
                .execute(&self.pool)
            })
            .collect::<Vec<_>>();

        future::try_join_all(insert_photo_query.into_iter()).await?;

        Ok(())
    }

    async fn delete(&self, gid: UserId) -> Result<()> {
        sqlx::query!(
            "UPDATE group_account SET is_deleted = true, deleted_at = ? WHERE gid = ?",
            Utc::now(),
            gid.to_string()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
