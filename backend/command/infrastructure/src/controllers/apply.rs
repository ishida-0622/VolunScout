use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use command_repository::activities::apply::ApplyRepository;
use domain::model::{
        apply::ApplyId,
        user_account::user_id::UserId, volunteer::VolunteerId
    };

use super::{WriteApiResponseFailureBody, WriteApiResponseSuccessBody, AppData};

/// ボランティア応募時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateApplyRequestBody {
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub uid: String,
    #[schema(required = true)]
    pub as_group: bool
}

/// 応募承認更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateApplyAllowedStatusRequestBody {
    #[schema(required = true)]
    pub aid: String,
    #[schema(required = true)]
    pub allowed_status: u8
}

/// 応募メール送信更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateApplyIsSentRequestBody {
    #[schema(required = true)]
    pub aid: String
}

#[utoipa::path(
    post,
    path="/apply/create",
    request_body=CreateApplyRequestBody,
    responses(
        (status=200, description="Create apply successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create apply failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn create_apply(
    State(state): State<AppData>,
    Json(body): Json<CreateApplyRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.apply_repository;

    let aid: ApplyId = ApplyId::new();
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

    let as_group: bool = body.as_group;

    match repository
        .create(aid, vid, uid, as_group)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create apply successfully.".to_string(),
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
    path="/apply/update/allowed-status",
    request_body=UpdateApplyAllowedStatusRequestBody,
    responses(
        (status=200, description="Update apply's allowed_status successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update apply's allowed_status failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_apply_allowed_status(
    State(state): State<AppData>,
    Json(body): Json<UpdateApplyAllowedStatusRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.apply_repository;

    let aid: ApplyId = ApplyId::from_str(&body.aid);

    let allowed_status: u8 = body.allowed_status;

    match repository
        .update_allowed_status(aid, allowed_status)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update apply's allowed_status successfully.".to_string(),
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
    path="/apply/update/is-sent",
    request_body=UpdateApplyIsSent,
    responses(
        (status=200, description="Update apply's is-sent successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update apply's is-sent failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_apply_is_sent(
    State(state): State<AppData>,
    Json(body): Json<UpdateApplyIsSentRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.apply_repository;

    let aid: ApplyId = ApplyId::from_str(&body.aid);

    match repository.update_is_sent(aid).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update apply's is_sent successfully.".to_string(),
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
