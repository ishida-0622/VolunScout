use anyhow::Result;
use async_trait::async_trait;
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
        phone: UserPhone,
        address: String,
        contents: String,
    ) -> Result<()> {
        sqlx::query!(
            "INSERT INTO group_account (gid, name, furigana, phone, address, contents) VALUES (?, ?, ?, ?, ?, ?)",
            gid.to_string(),
            name.to_string(),
            furigana.to_string(),
            phone.to_string(),
            address,
            contents
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(
        &self,
        gid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        address: String,
        contents: String,
    ) -> Result<()> {
        sqlx::query!(
            "UPDATE group_account SET name = ?, furigana = ?, phone = ?, address = ?, contents = ? WHERE gid = ?",
            name.to_string(),
            furigana.to_string(),
            phone.to_string(),
            address,
            contents,
            gid.to_string()
        )
        .execute(&self.pool)
        .await?;
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

    // async fn find_by_id(&self, gid: &UserId) -> Result<GroupAccount> {
    //     let group: GroupAccount = sqlx::query_as!(
    //         GroupAccount,
    //         "SELECT * FROM group_account WHERE gid = ?",
    //         gid.to_string()
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;
    //     Ok(group)
    // }

    // async fn find_by_ids(&self, gids: &[UserId]) -> Result<Vec<GroupAccount>> {
    //     let groups = sqlx::query_as!(
    //         GroupAccount,
    //         "SELECT * FROM group_account WHERE gid IN (?)",
    //         gids.iter()
    //             .map(|gid| gid.to_string())
    //             .collect::<Vec<String>>()
    //             .join(",")
    //     )
    //     .fetch_all(&self.pool)
    //     .await?;
    //     Ok(groups)
    // }

    // async fn find_all(&self) -> Result<Vec<GroupAccount>> {
    //     let groups = sqlx::query_as!(GroupAccount, "SELECT * FROM group_account")
    //         .fetch_all(&self.pool)
    //         .await?;
    //     Ok(groups)
    // }
}
