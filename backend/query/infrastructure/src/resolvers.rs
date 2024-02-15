use std::{str::FromStr, sync::Arc};

use anyhow::Result;
use async_graphql::{
    futures_util::{Stream, StreamExt},
    Context, EmptyMutation, Object, Schema, SchemaBuilder, Subscription,
};
use redis::Client;
use sqlx::MySqlPool;

use domain::model::{
    apply::ApplyId, scout::ScoutId, user_account::user_id::UserId, volunteer::VolunteerId,
};
use query_repository::{
    activities::{
        apply::{Apply, ApplyRepository, PastVolunteerParticipantReadModel},
        review::{
            ParticipantReviewPointAverage, ParticipantReviewRepository, Review,
            VolunteerReviewRepository,
        },
        scout::{Scout, ScoutRepository},
        volunteer::{VolunteerElementsReadModel, VolunteerQueryRepository, VolunteerReadModel},
    },
    user_account::{
        group::{GroupAccount, GroupUserRepository},
        participant::{
            GroupParticipant, ParticipantAccount, ParticipantCondition, ParticipantRegion,
            ParticipantTargetStatus, ParticipantTheme, ParticipantUserRepository, ScoutParticipant,
        },
    },
};

use crate::{
    activities::{
        apply::ApplyImpl, review::ReviewImpl, scout::ScoutImpl,
        volunteer::VolunteerQueryRepositoryImpl,
    },
    user_account::{group::GroupAccountImpl, participant::ParticipantAccountImpl},
};

pub struct ServiceContext {
    group_account_dao: Arc<dyn GroupUserRepository>,
    participant_account_dao: Arc<dyn ParticipantUserRepository>,
    scout_dao: Arc<dyn ScoutRepository>,
    apply_dao: Arc<dyn ApplyRepository>,
    volunteer_dao: Arc<dyn VolunteerQueryRepository>,
    participant_review_dao: Arc<dyn ParticipantReviewRepository>,
    volunteer_review_dao: Arc<dyn VolunteerReviewRepository>,
}

impl ServiceContext {
    pub fn new(
        group_account_dao: Arc<dyn GroupUserRepository>,
        participant_account_dao: Arc<dyn ParticipantUserRepository>,
        scout_dao: Arc<dyn ScoutRepository>,
        apply_dao: Arc<dyn ApplyRepository>,
        volunteer_dao: Arc<dyn VolunteerQueryRepository>,
        participant_review_dao: Arc<dyn ParticipantReviewRepository>,
        volunteer_review_dao: Arc<dyn VolunteerReviewRepository>,
    ) -> Self {
        Self {
            group_account_dao,
            participant_account_dao,
            scout_dao,
            apply_dao,
            volunteer_dao,
            participant_review_dao,
            volunteer_review_dao,
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

    /// 団体アカウントの存在チェックをする
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `bool` - 存在する場合はtrue
    async fn exists_group_account<'ctx>(&self, ctx: &Context<'ctx>, gid: String) -> Result<bool> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid: UserId = UserId::new(&gid).unwrap();
        let exists: bool = ctx.group_account_dao.exists(&gid).await?;

        Ok(exists)
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

    /// 指定されたaidの集団応募者の詳細情報を取得する
    ///
    /// ## 引数
    /// - `aid` - aid
    ///
    /// ## 返り値
    /// - `Vec<GroupParticipant>` - 集団応募者の詳細情報の配列
    async fn get_group_participants<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        aid: String,
    ) -> Result<Vec<GroupParticipant>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let aid: ApplyId = ApplyId::from_str(&aid);
        let participants: Vec<GroupParticipant> = ctx
            .participant_account_dao
            .find_group_participants(&aid)
            .await?;

        Ok(participants)
    }

    /// 指定されたsidのスカウト情報を取得する
    ///
    /// ## 引数
    /// - `sid` - sid
    ///
    /// ## 返り値
    /// - `Scout` - スカウト情報
    async fn get_scout_by_sid<'ctx>(&self, ctx: &Context<'ctx>, sid: String) -> Result<Scout> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let sid: ScoutId = ScoutId::from_str(&sid);
        let scout: Scout = ctx.scout_dao.find_by_sid(&sid).await?;

        Ok(scout)
    }

    /// 指定されたgidの団体が登録したボランティアのスカウト情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `Vec<Scout>` - スカウト情報の配列
    async fn get_scout_by_gid<'ctx>(&self, ctx: &Context<'ctx>, gid: String) -> Result<Vec<Scout>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid: UserId = UserId::new(&gid).unwrap();
        let scout: Vec<Scout> = ctx.scout_dao.find_by_gid(&gid).await?;

        Ok(scout)
    }

    /// 指定されたvidのスカウト情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Vec<Scout>` - スカウト情報の配列
    async fn get_scout_by_vid<'ctx>(&self, ctx: &Context<'ctx>, vid: String) -> Result<Vec<Scout>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid: VolunteerId = VolunteerId::from_str(&vid);
        let scout: Vec<Scout> = ctx.scout_dao.find_by_vid(&vid).await?;

        Ok(scout)
    }

    /// 指定されたuidに送られたスカウト情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<Scout>` - スカウト情報の配列
    async fn get_scout_by_uid<'ctx>(&self, ctx: &Context<'ctx>, uid: String) -> Result<Vec<Scout>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let scout: Vec<Scout> = ctx.scout_dao.find_by_uid(&uid).await?;

        Ok(scout)
    }

    /// 指定されたaidの応募情報を取得する
    ///
    /// ## 引数
    /// - `aid` - aid
    ///
    /// ## 返り値
    /// - `Apply` - 応募情報
    async fn get_apply_by_aid<'ctx>(&self, ctx: &Context<'ctx>, aid: String) -> Result<Apply> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let aid: ApplyId = ApplyId::from_str(&aid);
        let apply: Apply = ctx.apply_dao.find_by_sid(&aid).await?;

        Ok(apply)
    }

    /// 指定されたgidの団体が登録したボランティアの応募情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `Vec<Apply>` - 応募情報の配列
    async fn get_apply_by_gid<'ctx>(&self, ctx: &Context<'ctx>, gid: String) -> Result<Vec<Apply>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid: UserId = UserId::new(&gid).unwrap();
        let scout: Vec<Apply> = ctx.apply_dao.find_by_gid(&gid).await?;

        Ok(scout)
    }

    /// 指定されたvidの応募情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Vec<Apply>` - 応募情報の配列
    async fn get_apply_by_vid<'ctx>(&self, ctx: &Context<'ctx>, vid: String) -> Result<Vec<Apply>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid: VolunteerId = VolunteerId::from_str(&vid);
        let scout: Vec<Apply> = ctx.apply_dao.find_by_vid(&vid).await?;

        Ok(scout)
    }

    /// 指定されたuidに送られた応募情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<Apply>` - 応募情報の配列
    async fn get_apply_by_uid<'ctx>(&self, ctx: &Context<'ctx>, uid: String) -> Result<Vec<Apply>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid: UserId = UserId::new(&uid).unwrap();
        let scout: Vec<Apply> = ctx.apply_dao.find_by_uid(&uid).await?;

        Ok(scout)
    }

    /// 指定されたvidのボランティアに参加した参加者情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Vec<PastVolunteerParticipantReadModel>` - 参加者情報の配列
    async fn get_past_volunteer_participants_by_vid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        vid: String,
    ) -> Result<Vec<PastVolunteerParticipantReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = VolunteerId::from_str(&vid);
        let participants: Vec<PastVolunteerParticipantReadModel> =
            ctx.apply_dao.find_past_volunteer_participants(&vid).await?;

        Ok(participants)
    }

    /// 指定されたvidのボランティア要素情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `VolunteerElementsReadModel` - ボランティア要素情報
    async fn get_volunteer_elements_by_id<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        vid: String,
    ) -> Result<VolunteerElementsReadModel> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = VolunteerId::from_str(&vid);
        let volunteer: VolunteerElementsReadModel =
            ctx.volunteer_dao.find_elements_by_id(&vid).await?;

        Ok(volunteer)
    }

    /// ボランティアを検索する
    ///
    /// ## 引数
    /// - `regions` - OR検索の地域: Vec<String>
    /// - `required_regions` - AND検索の地域(東京23区外のボランティアを除外する等の場合で使用を推奨): Vec<String>,
    /// - `themes` - OR検索のテーマ: Vec<String>,
    /// - `required_themes` - AND検索のテーマ: Vec<String>,
    /// - `conditions` - OR検索の条件: Vec<String>,
    /// - `required_conditions` - AND検索の条件: Vec<String>,
    /// - `target_status` - 対象者: Vec<String>,
    /// - `search_words` - 検索ワード: String
    ///
    /// ## 返り値
    /// - `VolunteerElementsReadModel` - ボランティア要素情報
    async fn search_volunteer_by_elements<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        regions: Vec<String>,
        required_regions: Vec<String>,
        themes: Vec<String>,
        required_themes: Vec<String>,
        conditions: Vec<String>,
        required_conditions: Vec<String>,
        target_status: Vec<String>,
        search_words: String
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = "";
        let volunteer: Vec<VolunteerReadModel> = ctx.volunteer_dao.find_by_elements(
            &VolunteerElementsReadModel::new(vid.to_string(), regions, Some(required_regions), themes, required_themes, conditions, required_conditions, target_status),
            search_words
        ).await?;

        Ok(volunteer)
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

    /// 指定されたuidがお気に入りに登録しているボランティア情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_favorite_by_uid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let volunteers: Vec<VolunteerReadModel> =
            ctx.volunteer_dao.find_favorite_by_id(&uid).await?;

        Ok(volunteers)
    }

    /// 指定されたuidが過去に活動したボランティア情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_activities_by_uid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let volunteers: Vec<VolunteerReadModel> =
            ctx.volunteer_dao.find_activity_by_id(&uid).await?;

        Ok(volunteers)
    }

    /// 指定されたuidがこれから活動を予定しているボランティア情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_scheduled_activities_by_uid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let volunteers: Vec<VolunteerReadModel> = ctx
            .volunteer_dao
            .find_scheduled_activity_by_id(&uid)
            .await?;

        Ok(volunteers)
    }

    /// 指定されたgidが過去に活動したボランティア情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_activities_by_gid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid = UserId::from_str(&gid).unwrap();
        let volunteers: Vec<VolunteerReadModel> =
            ctx.volunteer_dao.find_activity_by_gid(&gid).await?;

        Ok(volunteers)
    }

    /// 指定されたgidがこれから活動を予定しているボランティア情報を取得する
    ///
    /// ## 引数
    /// - `gid` - gid
    ///
    /// ## 返り値
    /// - `Vec<VolunteerReadModel>` - ボランティア情報の配列
    async fn get_scheduled_activities_by_gid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        gid: String,
    ) -> Result<Vec<VolunteerReadModel>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let gid = UserId::from_str(&gid).unwrap();
        let volunteers: Vec<VolunteerReadModel> = ctx
            .volunteer_dao
            .find_scheduled_activity_by_gid(&gid)
            .await?;

        Ok(volunteers)
    }

    /// 指定されたuidとvidの参加者レビュー情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_participant_review_by_ids<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
        vid: String,
    ) -> Result<Review> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let vid = VolunteerId::from_str(&vid);
        let review: Review = ctx.participant_review_dao.find_by_ids(&uid, &vid).await?;

        Ok(review)
    }

    /// 指定されたuidの参加者レビュー情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_participant_review_by_uid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<Review>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let review: Vec<Review> = ctx.participant_review_dao.find_by_uid(&uid).await?;

        Ok(review)
    }

    /// 指定されたvidの参加者レビュー情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_participant_review_by_vid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        vid: String,
    ) -> Result<Vec<Review>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = VolunteerId::from_str(&vid);
        let review: Vec<Review> = ctx.participant_review_dao.find_by_vid(&vid).await?;

        Ok(review)
    }

    /// 指定されたuidとvidのボランティアレビュー情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_volunteer_review_by_ids<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
        vid: String,
    ) -> Result<Review> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let vid = VolunteerId::from_str(&vid);
        let review: Review = ctx.volunteer_review_dao.find_by_ids(&uid, &vid).await?;

        Ok(review)
    }

    /// 指定されたuidのボランティアレビュー情報を取得する
    ///
    /// ## 引数
    /// - `uid` - uid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_volunteer_review_by_uid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uid: String,
    ) -> Result<Vec<Review>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uid = UserId::from_str(&uid).unwrap();
        let review: Vec<Review> = ctx.volunteer_review_dao.find_by_uid(&uid).await?;

        Ok(review)
    }

    /// 指定されたvidのボランティアレビュー情報を取得する
    ///
    /// ## 引数
    /// - `vid` - vid
    ///
    /// ## 返り値
    /// - `Review` - レビュー情報
    async fn get_volunteer_review_by_vid<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        vid: String,
    ) -> Result<Vec<Review>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = VolunteerId::from_str(&vid);
        let review: Vec<Review> = ctx.volunteer_review_dao.find_by_vid(&vid).await?;

        Ok(review)
    }

    /// 指定されたuidの参加者レビュー情報の平均を取得する
    ///
    /// ## 引数
    /// - `uids` - uidの配列
    ///
    /// ## 返り値
    /// - `Vec<ParticipantReviewPointAverage>` - レビュー情報の配列
    async fn get_participant_review_average_by_uids<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        uids: Vec<String>,
    ) -> Result<Vec<ParticipantReviewPointAverage>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let uids: Vec<UserId> = uids
            .iter()
            .map(|uid| UserId::from_str(uid).unwrap())
            .collect();

        let reviews: Vec<ParticipantReviewPointAverage> =
            ctx.participant_review_dao.find_by_uids(&uids).await?;

        Ok(reviews)
    }

    /// スカウトに適した参加者を検索する
    ///
    /// ## 引数
    /// - `regions` - 地域: Vec<String>
    /// - `themes` - 設定したテーマ: Vec<String>,
    /// - `required_themes` - 必須で設定したテーマ: Vec<String>,
    /// - `conditions` - 設定した条件: Vec<String>,
    /// - `required_conditions` - 必須で設定した条件: Vec<String>,
    /// - `target_status` - 対象者: Vec<String>
    ///
    /// ## 返り値
    /// - `ScoutParticipant` - スカウトに必要な参加者情報
    async fn scout_participant_by_elements<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        regions: Vec<String>,
        themes: Vec<String>,
        required_themes: Vec<String>,
        conditions: Vec<String>,
        required_conditions: Vec<String>,
        target_status: Vec<String>
    ) -> Result<Vec<ScoutParticipant>> {
        let ctx: &ServiceContext = ctx.data::<ServiceContext>().unwrap();
        let vid = "";
        let participants: Vec<ScoutParticipant> = ctx.participant_account_dao.find_by_elements(
            &VolunteerElementsReadModel::new(vid.to_string(), regions, None, themes, required_themes, conditions, required_conditions, target_status)
        ).await?;

        Ok(participants)
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
    let scout_dao: ScoutImpl = ScoutImpl::new(pool.clone());
    let apply_dao: ApplyImpl = ApplyImpl::new(pool.clone());
    let volunteer_dao: VolunteerQueryRepositoryImpl =
        VolunteerQueryRepositoryImpl::new(pool.clone());
    let participant_review_dao: ReviewImpl = ReviewImpl::new(pool.clone());
    let volunteer_review_dao: ReviewImpl = ReviewImpl::new(pool.clone());

    let ctx: ServiceContext = ServiceContext::new(
        Arc::new(group_account_dao),
        Arc::new(participant_account_dao),
        Arc::new(scout_dao),
        Arc::new(apply_dao),
        Arc::new(volunteer_dao),
        Arc::new(participant_review_dao),
        Arc::new(volunteer_review_dao),
    );

    create_schema_builder().data(ctx).finish()
}
