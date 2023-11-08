use std::sync::Arc;

use anyhow::Result;
use async_graphql::futures_util::Stream;
use async_graphql::futures_util::StreamExt;
use async_graphql::{Context, EmptyMutation, Object, Schema, SchemaBuilder, Subscription};
use redis::Client;
use sqlx::MySqlPool;

use domain::repository::user_account::{GroupAccount, GroupUserRepository};

pub struct ServiceContext {
    user_account_dao: Arc<dyn GroupUserRepository>,
}

/// クエリ
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_group_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gid: String,
    ) -> Result<GroupAccount> {
        let ctx = ctx.data::<ServiceContext>().unwrap();
        let gid = domain::model::user_account::user_id::UserId::new(&gid).unwrap();
        let group_account = ctx.user_account_dao.find_by_id(&gid).await?;

        Ok(group_account)
    }
}
