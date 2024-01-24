use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use command_repository::activities::scout::ScoutRepository;
use domain::model::{
        scout::ScoutId,
        user_account::user_id::UserId, volunteer::VolunteerId
    };

use super::{WriteApiResponseFailureBody, WriteApiResponseSuccessBody, AppData};

/// スカウト時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateScoutRequestBody {
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub uid: String,
    #[schema(required = true)]
    pub message: String
}

/// スカウトメール送信済み更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateScoutIsSentRequestBody {
    #[schema(required = true)]
    pub sid: String
}

/// スカウト既読更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateScoutIsReadRequestBody {
    #[schema(required = true)]
    pub sid: String
}

/// スカウト拒否時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateScoutDeniedRequestBody {
    #[schema(required = true)]
    pub sid: String,
}

#[utoipa::path(
    post,
    path="/scout/create",
    request_body=CreateScoutRequestBody,
    responses(
        (status=200, description="Create scout successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create scout failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn create_scout(
    State(state): State<AppData>,
    Json(body): Json<CreateScoutRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.scout_repository;

    let sid: ScoutId = ScoutId::new();
    let vid: VolunteerId = VolunteerId::from_str(&body.vid);

    let uid: UserId = match UserId::from_str(&body.uid) {
        Ok(uid) => uid,
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

    let message: String = body.message;

    match repository
        .create(sid, vid, uid, message)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create scout successfully.".to_string(),
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
    path="/scout/update/is-sent",
    request_body=UpdateScoutIsSent,
    responses(
        (status=200, description="Update scout's is_sent successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update scout's is_sent failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_scout_is_sent(
    State(state): State<AppData>,
    Json(body): Json<UpdateScoutIsSentRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.scout_repository;

    let sid: ScoutId = ScoutId::from_str(&body.sid);

    match repository.update_is_sent(sid).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update scout's is_sent successfully.".to_string(),
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
    path="/scout/update/is-read",
    request_body=UpdateScoutIsReadRequestBody,
    responses(
        (status=200, description="Update scout's is_read successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update scout's is_read failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_scout_is_read(
    State(state): State<AppData>,
    Json(body): Json<UpdateScoutIsReadRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.scout_repository;

    let sid: ScoutId = ScoutId::from_str(&body.sid);

    match repository
        .update_is_read(sid)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update scout's is_read successfully.".to_string(),
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
    path="/scout/update/denied",
    request_body=UpdateScoutDeniedRequestBody,
    responses(
        (status=200, description="Update scout's denied successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update scout's denied failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_scout_denied(
    State(state): State<AppData>,
    Json(body): Json<UpdateScoutDeniedRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.scout_repository;

    let sid: ScoutId = ScoutId::from_str(&body.sid);

    match repository
        .update_denied(sid)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update scout's denied successfully.".to_string(),
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
