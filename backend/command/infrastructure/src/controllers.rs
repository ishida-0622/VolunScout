use std::{str::FromStr, sync::Arc};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use chrono::NaiveDate;
use lambda_http::Body;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tokio::sync::RwLock;
use utoipa::ToSchema;

use command_repository::user_account::{
    group::GroupUserRepository, participant::ParticipantUserRepository,
};
use domain::model::{
    gender::{gender_from_i8, Gender},
    region::Region,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
};

use crate::user_account::{group::GroupAccountImpl, participant::ParticipantAccountImpl};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WriteApiResponseFailureBody {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WriteApiResponseSuccessBody {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub address: String,
    #[schema(required = true)]
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub address: String,
    #[schema(required = true)]
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateParticipantAccountRequestBody {
    #[schema(required = true)]
    pub pid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub gender: i8,
    #[schema(required = true)]
    pub birthday: NaiveDate,
    #[schema(required = true)]
    pub region: Vec<String>,
    #[schema(required = true)]
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateParticipantAccountRequestBody {
    #[schema(required = true)]
    pub pid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub gender: i8,
    #[schema(required = true)]
    pub birthday: NaiveDate,
    #[schema(required = true)]
    pub region: Vec<String>,
    #[schema(required = true)]
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteParticipantAccountRequestBody {
    #[schema(required = true)]
    pub pid: String,
}

pub struct AppState {
    group_account_repository: GroupAccountImpl,
    participant_account_repository: ParticipantAccountImpl,
}

impl AppState {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            group_account_repository: GroupAccountImpl::new(pool.clone()),
            participant_account_repository: ParticipantAccountImpl::new(pool.clone()),
        }
    }
}

pub type AppData = Arc<RwLock<AppState>>;

#[utoipa::path(
    post,
    path="/group-account/create",
    request_body=CreateGroupAccountRequestBody,
    responses(
        (status=200, description="Create group account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create group account failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn create_group_account(
    State(state): State<AppData>,
    Json(body): Json<CreateGroupAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.group_account_repository;

    let gid: UserId = match UserId::from_str(&body.gid) {
        Ok(gid) => gid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let name: UserName = match UserName::from_str(&body.name) {
        Ok(name) => name,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.furigana) {
        Ok(furigana) => furigana,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let phone: UserPhone = match UserPhone::from_str(&body.phone) {
        Ok(phone) => phone,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let address: String = body.address;

    let contents: String = body.contents;

    match repository
        .create(gid, name, furigana, phone, address, contents)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create group account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path="/group-account/update",
    request_body=UpdateGroupAccountRequestBody,
    responses(
        (status=200, description="Update group account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update group account failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_group_account(
    State(state): State<AppData>,
    Json(body): Json<UpdateGroupAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.group_account_repository;

    let gid: UserId = match UserId::from_str(&body.gid) {
        Ok(gid) => gid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let name: UserName = match UserName::from_str(&body.name) {
        Ok(name) => name,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.furigana) {
        Ok(furigana) => furigana,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let phone: UserPhone = match UserPhone::from_str(&body.phone) {
        Ok(phone) => phone,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let address: String = body.address;

    let contents: String = body.contents;

    match repository
        .update(gid, name, furigana, phone, address, contents)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update group account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path="/group-account/delete",
    request_body=DeleteGroupAccountRequestBody,
    responses(
        (status=200, description="Delete group account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Delete group account failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn delete_group_account(
    State(state): State<AppData>,
    Json(body): Json<DeleteGroupAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.group_account_repository;

    let gid: UserId = match UserId::from_str(&body.gid) {
        Ok(gid) => gid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    match repository.delete(gid).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Delete group account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path="/participant-account/create",
    request_body=CreateParticipantAccountRequestBody,
    responses(
        (status=200, description="Create participant account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create participant account failed.", body=WriteApiResponseFailureBody)
    )
)]
async fn create_participant_account(
    State(state): State<AppData>,
    Json(body): Json<CreateParticipantAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.participant_account_repository;

    let pid: UserId = match UserId::from_str(&body.pid) {
        Ok(pid) => pid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let name: UserName = match UserName::from_str(&body.name) {
        Ok(name) => name,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.furigana) {
        Ok(furigana) => furigana,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let phone: UserPhone = match UserPhone::from_str(&body.phone) {
        Ok(phone) => phone,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let gender: Gender = match gender_from_i8(&body.gender) {
        Ok(gender) => gender,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let birthday: NaiveDate = body.birthday;

    let region: Vec<Region> = body
        .region
        .iter()
        .map(|r: &String| Region::from_str(r).unwrap())
        .collect::<Vec<Region>>();

    let profile: String = body.profile;

    match repository
        .create(
            pid, name, furigana, phone, gender, birthday, region, profile,
        )
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create participant account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path="/participant-account/update",
    request_body=UpdateParticipantAccountRequestBody,
    responses(
        (status=200, description="Update participant account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update participant account failed.", body=WriteApiResponseFailureBody)
    )
)]
async fn update_participant_account(
    State(state): State<AppData>,
    Json(body): Json<UpdateParticipantAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.participant_account_repository;

    let pid: UserId = match UserId::from_str(&body.pid) {
        Ok(pid) => pid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let name: UserName = match UserName::from_str(&body.name) {
        Ok(name) => name,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.furigana) {
        Ok(furigana) => furigana,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let phone: UserPhone = match UserPhone::from_str(&body.phone) {
        Ok(phone) => phone,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let gender: Gender = match gender_from_i8(&body.gender) {
        Ok(gender) => gender,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    let birthday: NaiveDate = body.birthday;

    let region: Vec<Region> = body
        .region
        .iter()
        .map(|r: &String| Region::from_str(r).unwrap())
        .collect::<Vec<Region>>();

    let profile: String = body.profile;

    match repository
        .update(
            pid, name, furigana, phone, gender, birthday, region, profile,
        )
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update participant account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post,
    path="/participant-account/delete",
    request_body=DeleteParticipantAccountRequestBody,
    responses(
        (status=200, description="Delete participant account successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Delete participant account failed.", body=WriteApiResponseFailureBody)
    )
)]
async fn delete_participant_account(
    State(state): State<AppData>,
    Json(body): Json<DeleteParticipantAccountRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.participant_account_repository;

    let pid: UserId = match UserId::from_str(&body.pid) {
        Ok(pid) => pid,
        Err(error) => {
            log::warn!("error = {}", error);
            return (
                StatusCode::BAD_REQUEST,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response();
        }
    };

    match repository.delete(pid).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Delete participant account successfully.".to_string(),
            }),
        )
            .into_response(),
        Err(error) => {
            log::error!("error = {}", error);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(WriteApiResponseFailureBody {
                    message: error.to_string(),
                }),
            )
                .into_response()
        }
    }
}

pub enum Endpoints {
    CreateGroupAccount,
    UpdateGroupAccount,
    DeleteGroupAccount,
    CreateParticipantAccount,
    UpdateParticipantAccount,
    DeleteParticipantAccount,
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
        }
    }
}

// ローカルで動かすときはこっち
pub fn create_router(pool: MySqlPool) -> Router {
    // Lambdaで動かすときはこっち
    // pub fn create_router(pool: MySqlPool) -> Router<(), Body> {
    let state: Arc<RwLock<AppState>> = Arc::new(RwLock::new(AppState::new(pool)));

    let router = Router::new()
        .route(
            Endpoints::CreateGroupAccount.as_str(),
            post(create_group_account),
        )
        .route(
            Endpoints::UpdateGroupAccount.as_str(),
            post(update_group_account),
        )
        .route(
            Endpoints::DeleteGroupAccount.as_str(),
            post(delete_group_account),
        )
        .route(
            Endpoints::CreateParticipantAccount.as_str(),
            post(create_participant_account),
        )
        .route(
            Endpoints::UpdateParticipantAccount.as_str(),
            post(update_participant_account),
        )
        .route(
            Endpoints::DeleteParticipantAccount.as_str(),
            post(delete_participant_account),
        )
        .with_state(state);

    router
}
