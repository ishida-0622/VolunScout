pub mod group;
pub mod participant;
pub mod volunteer;

use axum::{routing::post, Router};
use lambda_http::Body;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::activities::volunteer::VolunteerImpl;
use crate::user_account::{group::GroupAccountImpl, participant::ParticipantAccountImpl};

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
}

impl AppState {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            group_account_repository: GroupAccountImpl::new(pool.clone()),
            participant_account_repository: ParticipantAccountImpl::new(pool.clone()),
            volunteer_repository: VolunteerImpl::new(pool.clone()),
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
        .with_state(state);

    router
}
