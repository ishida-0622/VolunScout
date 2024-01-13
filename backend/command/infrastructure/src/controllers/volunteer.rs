use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, NaiveDate, Utc};

use command_repository::activities::volunteer::VolunteerRepository;
use domain::model::{
        volunteer::VolunteerId,
        user_account::user_id::UserId,
        terms::Terms, region::Region, theme::Theme, target_status::TargetStatus, condition::Condition
    };

use super::{WriteApiResponseFailureBody, WriteApiResponseSuccessBody, AppData};

/// ボランティアの作成時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateVolunteerRequestBody {
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub title: String,
    #[schema(required = true)]
    pub message: String,
    #[schema(required = true)]
    pub overview: String,
    #[schema(required = true)]
    pub recruited_num: u32,
    #[schema(required = true)]
    pub place: String,
    #[schema(required = true, value_type = String, example = "2023-12-17T9:00:00Z")]
    pub start_at: DateTime<Utc>,
    #[schema(required = true, value_type = String, example = "2023-12-17T17:00:00Z")]
    pub finish_at: DateTime<Utc>,
    #[schema(required = true, value_type = String, example = "2023-12-3")]
    pub deadline_on: NaiveDate,
    #[schema(required = true)]
    pub as_group: bool,
    #[schema(required = true)]
    pub region: Vec<String>,
    #[schema(required = true)]
    pub theme: Vec<String>,
    #[schema(required = true)]
    pub required_theme: Vec<String>,
    #[schema(required = true)]
    pub condition: Vec<String>,
    #[schema(required = true)]
    pub required_condition: Vec<String>,
    #[schema()]
    pub reward: Option<String>,
    #[schema(required = true)]
    pub target_status: Vec<String>,
}

/// ボランティアの更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateVolunteerRequestBody {
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub title: String,
    #[schema(required = true)]
    pub message: String,
    #[schema(required = true)]
    pub overview: String,
    #[schema(required = true)]
    pub recruited_num: u32,
    #[schema(required = true)]
    pub place: String,
    #[schema(required = true, value_type = String, format = DateTime, example = "2023-12-17T09:00:00Z")]
    pub start_at: DateTime<Utc>,
    #[schema(required = true, value_type = String, format = DateTime, example = "2023-12-17T17:00:00Z")]
    pub finish_at: DateTime<Utc>,
    #[schema(required = true, value_type = String, example = "2023-12-3")]
    pub deadline_on: NaiveDate,
    #[schema(required = true)]
    pub as_group: bool,
    #[schema(required = true)]
    pub region: Vec<String>,
    #[schema(required = true)]
    pub theme: Vec<String>,
    #[schema(required = true)]
    pub required_theme: Vec<String>,
    #[schema(required = true)]
    pub condition: Vec<String>,
    #[schema(required = true)]
    pub required_condition: Vec<String>,
    #[schema()]
    pub reward: Option<String>,
    #[schema(required = true)]
    pub target_status: Vec<String>,
}

/// ボランティアの削除時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteVolunteerRequestBody {
    #[schema(required = true)]
    pub vid: String,
}

#[utoipa::path(
    post,
    path="/volunteer/create",
    request_body=CreateVolunteerRequestBody,
    responses(
        (status=200, description="Create volunteer successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Create volunteer failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn create_volunteer(
    State(state): State<AppData>,
    Json(body): Json<CreateVolunteerRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.volunteer_repository;

    let vid: VolunteerId = VolunteerId::new();

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

    let title: String = body.title;
    let message: String = body.message;
    let overview: String = body.overview;
    let recruited_num: u32 = body.recruited_num;
    let place: String = body.place;
    let start_at: DateTime<Utc> = body.start_at;
    let finish_at: DateTime<Utc> = body.finish_at;
    let deadline_on: NaiveDate = body.deadline_on;
    let as_group: bool = body.as_group;
    let region: Vec<Region> = body
        .region
        .iter()
        .map(|r: &String| Region::from_str(r).unwrap())
        .collect::<Vec<Region>>();
    let theme: Vec<Theme> = body
        .theme
        .iter()
        .map(|t: &String| Theme::from_str(t).unwrap())
        .collect::<Vec<Theme>>();
    let required_theme: Vec<Theme> = body
        .required_theme
        .iter()
        .map(|t: &String| Theme::from_str(t).unwrap())
        .collect::<Vec<Theme>>();
    let condition: Vec<Condition> = body
        .condition
        .iter()
        .map(|t: &String| Condition::from_str(t).unwrap())
        .collect::<Vec<Condition>>();
    let required_condition: Vec<Condition> = body
        .required_condition
        .iter()
        .map(|t: &String| Condition::from_str(t).unwrap())
        .collect::<Vec<Condition>>();
    let reward: Option<String> = body.reward;
    let target_status: Vec<TargetStatus> = body
        .target_status
        .iter()
        .map(|t: &String| TargetStatus::from_str(t).unwrap())
        .collect::<Vec<TargetStatus>>();
    let terms: Terms = Terms::new(
        region,
        theme,
        required_theme,
        condition,
        required_condition,
        // reward,
        target_status
    );

    match repository
        .create(vid, gid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group, reward, terms)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Create volunteer successfully.".to_string(),
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
    path="/volunteer/update",
    request_body=UpdateVolunteerRequestBody,
    responses(
        (status=200, description="Update volunteer successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Update volunteer failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn update_volunteer(
    State(state): State<AppData>,
    Json(body): Json<UpdateVolunteerRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.volunteer_repository;

    let vid: VolunteerId = VolunteerId::from_str(&body.vid);

    let title: String = body.title;
    let message: String = body.message;
    let overview: String = body.overview;
    let recruited_num: u32 = body.recruited_num;
    let place: String = body.place;

    let start_at: DateTime<Utc> = body.start_at;
    let finish_at: DateTime<Utc> = body.finish_at;
    let deadline_on: NaiveDate = body.deadline_on;
    let as_group: bool = body.as_group;
    let region: Vec<Region> = body
        .region
        .iter()
        .map(|r: &String| Region::from_str(r).unwrap())
        .collect::<Vec<Region>>();
    let theme: Vec<Theme> = body
        .theme
        .iter()
        .map(|t: &String| Theme::from_str(t).unwrap())
        .collect::<Vec<Theme>>();
    let required_theme: Vec<Theme> = body
        .required_theme
        .iter()
        .map(|t: &String| Theme::from_str(t).unwrap())
        .collect::<Vec<Theme>>();
    let condition: Vec<Condition> = body
        .condition
        .iter()
        .map(|t: &String| Condition::from_str(t).unwrap())
        .collect::<Vec<Condition>>();
    let required_condition: Vec<Condition> = body
        .required_condition
        .iter()
        .map(|t: &String| Condition::from_str(t).unwrap())
        .collect::<Vec<Condition>>();
    let reward: Option<String> = body.reward;
    let target_status: Vec<TargetStatus> = body
        .target_status
        .iter()
        .map(|t: &String| TargetStatus::from_str(t).unwrap())
        .collect::<Vec<TargetStatus>>();
    let terms: Terms = Terms::new(
        region,
        theme,
        required_theme,
        condition,
        required_condition,
        target_status
    );

    match repository
        .update(vid, title, message, overview, recruited_num, place, start_at, finish_at, deadline_on, as_group, reward, terms)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Update volunteer successfully.".to_string(),
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
    path="/volunteer/delete",
    request_body=DeleteVolunteerRequestBody,
    responses(
        (status=200, description="Delete volunteer successfully.", body=WriteApiResponseSuccessBody),
        (status=500, description="Delete volunteer failed.", body=WriteApiResponseFailureBody)
    )
)]
pub async fn delete_volunteer(
    State(state): State<AppData>,
    Json(body): Json<DeleteVolunteerRequestBody>,
) -> impl IntoResponse {
    let mut lock = state.write().await;
    let repository = &mut lock.volunteer_repository;

    let vid: VolunteerId = VolunteerId::from_str(&body.vid);

    match repository.delete(vid).await {
        Ok(_) => (
            StatusCode::OK,
            Json(WriteApiResponseSuccessBody {
                message: "Delete volunteer successfully.".to_string(),
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

