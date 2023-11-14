use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use async_graphql::{
    futures_util::{Stream, StreamExt},
    Context, EmptyMutation, Object, Schema, SchemaBuilder, Subscription,
};
use redis::Client;
use sqlx::MySqlPool;

use domain::model::user_account::user_id::UserId;
use query_repository::user_account::group::{GroupAccount, GroupUserRepository};

use crate::user_account::group::GroupAccountImpl;

pub struct ServiceContext {
    group_account_dao: Arc<dyn GroupUserRepository>,
}

impl ServiceContext {
    pub fn new(group_account_dao: Arc<dyn GroupUserRepository>) -> Self {
        Self { group_account_dao }
    }
}

/// クエリ
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// 指定されたgidのアカウント情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `GroupAccount` - グループアカウント情報
    async fn get_group_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gid: String,
    ) -> Result<GroupAccount> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid: UserId = UserId::new(&gid).unwrap();
        let group_account: GroupAccount = ctx.group_account_dao.find_by_id(&gid).await?;

        Ok(group_account)
    }

    /// 複数指定されたgidのアカウント情報を取得する
    ///
    /// ## 引数
    /// - `gids` - gidの配列
    ///
    /// ## 返り値
    /// - `Vec<GroupAccount>` - グループアカウント情報の配列
    async fn get_group_accounts<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gids: Vec<String>,
    ) -> Result<Vec<GroupAccount>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gids: Vec<UserId> = gids
            .iter()
            .map(|gid| UserId::from_str(gid).unwrap())
            .collect();

        let group_accounts: Vec<GroupAccount> = ctx.group_account_dao.find_by_ids(&gids).await?;

        Ok(group_accounts)
    }

    /// 全てのグループアカウント情報を取得する
    ///
    /// ## 返り値
    /// - `Vec<GroupAccount>` - グループアカウント情報の配列
    async fn get_all_group_accounts<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<GroupAccount>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let group_accounts: Vec<GroupAccount> = ctx.group_account_dao.find_all().await?;

        Ok(group_accounts)
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn foo<'ctx>(&self, ctx: &Context<'ctx>) -> Result<impl Stream<Item = String>> {
        let client = ctx.data_unchecked::<Client>();
        let mut conn = client.get_async_connection().await?.into_pubsub();
        conn.subscribe("foo").await?;
        Ok(conn
            .into_on_message()
            .filter_map(|msg| async move { msg.get_payload().ok() }))
    }
}

pub type ApiSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;

pub fn create_schema_builder() -> SchemaBuilder<QueryRoot, EmptyMutation, SubscriptionRoot> {
    Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
}

pub fn create_schema(pool: MySqlPool) -> ApiSchema {
    let group_account_dao: GroupAccountImpl = GroupAccountImpl::new(pool.clone());

    let ctx: ServiceContext = ServiceContext::new(Arc::new(group_account_dao));

    create_schema_builder().data(ctx).finish()
}
