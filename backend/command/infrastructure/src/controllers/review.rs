use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use command_repository::activities::review::ReviewRepository;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use domain::model::{
        user_account::user_id::UserId, volunteer::VolunteerId
    };

use super::{WriteApiResponseFailureBody, WriteApiResponseSuccessBody, AppData};

/// 参加者からボランティアへレビュー時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewToVolunteerRequestBody {
    #[schema(required = true)]
    pub uid: String,
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub point: u8,
    pub comment: Option<String>,
}

/// ボランティアから参加者へレビュー時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReviewToParticipantRequestBody {
    #[schema(required = true)]
    pub uid: String,
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub point: u8,
    pub comment: Option<String>,
}

#[utoipa::path(
    post,
    path="/review/to-volunteer",
    request_body=ReviewToVolunteerRequestBody,
    responses(
        (status=200, description="Create review (to volunteer) successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create review (to volunteer) failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn review_to_volunteer(
    State(state): State<AppData>,
    Json(body): Json<ReviewToVolunteerRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.review_repository;

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

    let vid: VolunteerId = VolunteerId::from_str(&body.vid);
    let point: u8 = body.point;
    let comment: Option<String> = body.comment;

    match repository
        .review_to_volunteer(uid, vid, point, comment)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create review(to-volunteer) successfully.".to_string(),
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
    path="/review/to-participant",
    request_body=ReviewToVolunteerRequestBody,
    responses(
        (status=200, description="Create review (to participant) successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create review (to participant) failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn review_to_participant(
    State(state): State<AppData>,
    Json(body): Json<ReviewToParticipantRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.review_repository;

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

    let vid: VolunteerId = VolunteerId::from_str(&body.vid);
    let point: u8 = body.point;
    let comment: Option<String> = body.comment;

    match repository
        .review_to_participant(uid, vid, point, comment)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create review(to-participant) successfully.".to_string(),
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
