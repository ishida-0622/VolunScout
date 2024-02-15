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
            r#"
            SELECT
                gid, name, furigana, phone, address, contents, representative_name, representative_furigana, is_paid as "is_paid: bool", is_deleted as "is_deleted: bool", deleted_at
            FROM group_account
            WHERE gid = ? AND is_deleted = false
            "#,
            gid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        if group.is_deleted {
            return Err(anyhow::anyhow!("this group_account is deleted but existed"));
        }
        Ok(group)
    }

    async fn find_by_ids(&self, gids: &[UserId]) -> Result<Vec<GroupAccount>> {
        // TODO: FIX: Where の IN が息してない
        let groups = sqlx::query_as!(
            GroupAccount,
            r#"
            SELECT
                gid, name, furigana, phone, address, contents, representative_name, representative_furigana, is_paid as "is_paid: bool", is_deleted as "is_deleted: bool", deleted_at
            FROM group_account
            WHERE gid IN (?) AND is_deleted = false
            "#,
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
        let groups = sqlx::query_as!(
            GroupAccount,
            r#"
            SELECT
                gid, name, furigana, phone, address, contents, representative_name, representative_furigana, is_paid as "is_paid: bool", is_deleted as "is_deleted: bool", deleted_at
            FROM group_account
            WHERE is_deleted = false
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(groups)
    }

    async fn exists(&self, gid: &UserId) -> Result<bool> {
        let exists = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT * FROM group_account WHERE gid = ? AND is_deleted = false) AS exist
            "#,
            gid.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(exists.exist == 1)
    }
}
