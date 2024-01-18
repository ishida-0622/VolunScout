pub mod group;
pub mod participant;
pub mod volunteer;
pub mod apply;
pub mod scout;

use axum::{routing::post, Router};
use lambda_http::Body;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::{
    activities::{volunteer::VolunteerImpl, apply::ApplyImpl, scout::ScoutImpl},
    user_account::{group::GroupAccountImpl, participant::ParticipantAccountImpl}
};

/// 失敗時のAPIレスポンスのボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WriteApiResponseFailureBody {
    pub message: String,
}

/// 成功時のAPIレスポンスのボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WriteApiResponseSuccessBody {
    pub message: String,
}

/// アプリケーションの状態を表す構造体
pub struct AppState {
    group_account_repository: GroupAccountImpl,
    participant_account_repository: ParticipantAccountImpl,
    volunteer_repository: VolunteerImpl,
    apply_repository: ApplyImpl,
    scout_repository: ScoutImpl
}

impl AppState {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            group_account_repository: GroupAccountImpl::new(pool.clone()),
            participant_account_repository: ParticipantAccountImpl::new(pool.clone()),
            volunteer_repository: VolunteerImpl::new(pool.clone()),
            apply_repository: ApplyImpl::new(pool.clone()),
            scout_repository: ScoutImpl::new(pool.clone())
        }
    }
}

/// アプリケーションの状態を保持するための型エイリアス
pub type AppData = Arc<RwLock<AppState>>;

/// APIエンドポイントを表す列挙型
pub enum Endpoints {
    CreateGroupAccount,
    UpdateGroupAccount,
    DeleteGroupAccount,
    CreateParticipantAccount,
    UpdateParticipantAccount,
    DeleteParticipantAccount,
    CreateVolunteer,
    UpdateVolunteer,
    DeleteVolunteer,
    RegisterVolunteerFavorite,
    UnregisterVolunteerFavorite,
    CreateApply,
    UpdateApplyAllowedStatus,
    UpdateApplyIsSent,
    CreateScout,
    UpdateScoutIsSent,
    UpdateScoutIsRead,
    UpdateScoutDenied,
}

impl Endpoints {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Endpoints::CreateGroupAccount => "/group-account/create",
            Endpoints::UpdateGroupAccount => "/group-account/update",
            Endpoints::DeleteGroupAccount => "/group-account/delete",
            Endpoints::CreateParticipantAccount => "/participant-account/create",
            Endpoints::UpdateParticipantAccount => "/participant-account/update",
            Endpoints::DeleteParticipantAccount => "/participant-account/delete",
            Endpoints::CreateVolunteer => "/volunteer/create",
            Endpoints::UpdateVolunteer => "/volunteer/update",
            Endpoints::DeleteVolunteer => "/volunteer/delete",
            Endpoints::RegisterVolunteerFavorite => "/volunteer/favorite/register",
            Endpoints::UnregisterVolunteerFavorite => "/volunteer/favorite/unregister",
            Endpoints::CreateApply => "/apply/create",
            Endpoints::UpdateApplyAllowedStatus => "/apply/update/allowed-status",
            Endpoints::UpdateApplyIsSent => "/apply/update/is-sent",
            Endpoints::CreateScout => "/scout/create",
            Endpoints::UpdateScoutIsSent => "/scout/update/is-sent",
            Endpoints::UpdateScoutIsRead => "/scout/update/is-read",
            Endpoints::UpdateScoutDenied => "/scout/update/denied",
        }
    }
}

pub fn create_router(pool: MySqlPool) -> Router {
    // Lambdaで動かす場合
    // pub fn create_router(pool: MySqlPool) -> Router<(), Body> {
    let state: Arc<RwLock<AppState>> = Arc::new(RwLock::new(AppState::new(pool)));

    let router = Router::new()
        .route(
            Endpoints::CreateGroupAccount.as_str(),
            post(group::create_group_account),
        )
        .route(
            Endpoints::UpdateGroupAccount.as_str(),
            post(group::update_group_account),
        )
        .route(
            Endpoints::DeleteGroupAccount.as_str(),
            post(group::delete_group_account),
        )
        .route(
            Endpoints::CreateParticipantAccount.as_str(),
            post(participant::create_participant_account),
        )
        .route(
            Endpoints::UpdateParticipantAccount.as_str(),
            post(participant::update_participant_account),
        )
        .route(
            Endpoints::DeleteParticipantAccount.as_str(),
            post(participant::delete_participant_account),
        )
        .route(
            Endpoints::CreateVolunteer.as_str(),
            post(volunteer::create_volunteer),
        )
        .route(
            Endpoints::UpdateVolunteer.as_str(),
            post(volunteer::update_volunteer),
        )
        .route(
            Endpoints::DeleteVolunteer.as_str(),
            post(volunteer::delete_volunteer),
        )
        .route(
            Endpoints::RegisterVolunteerFavorite.as_str(),
            post(volunteer::register_favorite),
        )
        .route(
            Endpoints::UnregisterVolunteerFavorite.as_str(),
            post(volunteer::unregister_favorite),
        )
        .route(
            Endpoints::CreateApply.as_str(),
            post(apply::create_apply),
        )
        .route(
            Endpoints::UpdateApplyAllowedStatus.as_str(),
            post(apply::update_apply_allowed_status),
        )
        .route(
            Endpoints::UpdateApplyIsSent.as_str(),
            post(apply::update_apply_is_sent),
        )
        .route(
            Endpoints::CreateScout.as_str(),
            post(scout::create_scout),
        )
        .route(
            Endpoints::UpdateScoutIsSent.as_str(),
            post(scout::update_scout_is_sent),
        )
        .route(
            Endpoints::UpdateScoutIsRead.as_str(),
            post(scout::update_scout_is_read),
        )
        .route(
            Endpoints::UpdateScoutDenied.as_str(),
            post(scout::update_scout_denied),
        )
        .with_state(state);

    router
}
