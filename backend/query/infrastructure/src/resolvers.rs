use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use async_graphql::{
    futures_util::{Stream, StreamExt},
    Context, EmptyMutation, Object, Schema, SchemaBuilder, Subscription,
};
use redis::Client;
use sqlx::MySqlPool;

use domain::model::{user_account::user_id::UserId, volunteer::VolunteerId};
use query_repository::{
    user_account::{
        group::{GroupAccount, GroupUserRepository},
        participant::{
            ParticipantAccount, ParticipantCondition, ParticipantRegion, ParticipantTargetStatus,
            ParticipantTheme, ParticipantUserRepository,
        },
    },
    volunteer::{VolunteerQueryRepository, VolunteerReadModel},
};

use crate::{
    user_account::{group::GroupAccountImpl, participant::ParticipantAccountImpl},
    volunteer::VolunteerQueryRepositoryImpl,
};

pub struct ServiceContext {
    group_account_dao: Arc<dyn GroupUserRepository>,
    participant_account_dao: Arc<dyn ParticipantUserRepository>,
    volunteer_dao: Arc<dyn VolunteerQueryRepository>,
}

impl ServiceContext {
    pub fn new(
        group_account_dao: Arc<dyn GroupUserRepository>,
        participant_account_dao: Arc<dyn ParticipantUserRepository>,
        volunteer_dao: Arc<dyn VolunteerQueryRepository>,
    ) -> Self {
        Self {
            group_account_dao,
            participant_account_dao,
            volunteer_dao,
        }
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

    /// 指定されたuidのアカウント情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `ParticipantAccount` - 参加者アカウント情報
    async fn get_participant_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<ParticipantAccount> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let participant_account: ParticipantAccount =
            ctx.participant_account_dao.find_by_id(&uid).await?;

        Ok(participant_account)
    }

    /// 複数指定されたuidのアカウント情報を取得する
    ///
    /// ## 引数
    /// - `uids` - uidの配列
    ///
    /// ## 返り値
    /// - `Vec<ParticipantAccount>` - 参加者アカウント情報の配列
    async fn get_participant_accounts<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uids: Vec<String>,
    ) -> Result<Vec<ParticipantAccount>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uids: Vec<UserId> = uids
            .iter()
            .map(|uid| UserId::from_str(uid).unwrap())
            .collect();

        let participant_accounts: Vec<ParticipantAccount> =
            ctx.participant_account_dao.find_by_ids(&uids).await?;

        Ok(participant_accounts)
    }

    /// 指定されたuidの地域情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<ParticipantRegion>` - 地域情報の配列
    async fn get_participant_regions<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<ParticipantRegion>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let regions: Vec<ParticipantRegion> =
            ctx.participant_account_dao.find_region_by_id(&uid).await?;

        Ok(regions)
    }

    /// 指定されたuidのテーマ情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<ParticipantTheme>` - テーマ情報の配列
    async fn get_participant_themes<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<ParticipantTheme>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let themes: Vec<ParticipantTheme> =
            ctx.participant_account_dao.find_theme_by_id(&uid).await?;

        Ok(themes)
    }

    /// 指定されたuidの条件情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<ParticipantCondition>` - 条件情報の配列
    async fn get_participant_conditions<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<ParticipantCondition>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let conditions: Vec<ParticipantCondition> = ctx
            .participant_account_dao
            .find_condition_by_id(&uid)
            .await?;

        Ok(conditions)
    }

    /// 指定されたuidの区分情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `ParticipantTargetStatus` - 対象状況情報
    async fn get_participant_target_status<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<ParticipantTargetStatus> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let target_status: ParticipantTargetStatus = ctx
            .participant_account_dao
            .find_target_status_by_id(&uid)
            .await?;

        Ok(target_status)
    }

    /// 参加者アカウントの存在チェックをする
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `bool` - 存在する場合はtrue
    async fn exists_participant_account<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<bool> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let exists: bool = ctx.participant_account_dao.exists(&uid).await?;

        Ok(exists)
    }

    /// 指定されたvidのボランティア情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `VolunteerReadModel` - ボランティア情報
    async fn get_volunteer_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        vid: String,
    ) -> Result<VolunteerReadModel> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = VolunteerId::from_str(&vid);
        let volunteer: VolunteerReadModel = ctx.volunteer_dao.find_by_id(&vid).await?;

        Ok(volunteer)
    }

    /// 指定されたgidのボランティア情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_volunteer_by_gid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid = UserId::from_str(&gid).unwrap();
        let volunteers: Vec<VolunteerReadModel> = ctx.volunteer_dao.find_by_gid(&gid).await?;

        Ok(volunteers)
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
    let participant_account_dao: ParticipantAccountImpl = ParticipantAccountImpl::new(pool.clone());
    let volunteer_dao: VolunteerQueryRepositoryImpl =
        VolunteerQueryRepositoryImpl::new(pool.clone());

    let ctx: ServiceContext = ServiceContext::new(
        Arc::new(group_account_dao),
        Arc::new(participant_account_dao),
        Arc::new(volunteer_dao),
    );

    create_schema_builder().data(ctx).finish()
}
