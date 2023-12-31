use anyhow::Result;
use async_trait::async_trait;
use sqlx::MySqlPool;

use domain::model::user_account::user_id::UserId;
use query_repository::user_account::group::{GroupAccount, GroupUserRepository};

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
    async fn find_by_id(&self, gid: &UserId) -> Result<GroupAccount> {
        let group: GroupAccount = sqlx::query_as!(
            GroupAccount,
            "SELECT * FROM group_account WHERE gid = ?",
            gid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(group)
    }

    async fn find_by_ids(&self, gids: &[UserId]) -> Result<Vec<GroupAccount>> {
        let groups = sqlx::query_as!(
            GroupAccount,
            "SELECT * FROM group_account WHERE gid IN (?)",
            gids.iter()
                .map(|gid| gid.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(groups)
    }

    async fn find_all(&self) -> Result<Vec<GroupAccount>> {
        let groups = sqlx::query_as!(GroupAccount, "SELECT * FROM group_account")
            .fetch_all(&self.pool)
            .await?;
        Ok(groups)
    }
}
