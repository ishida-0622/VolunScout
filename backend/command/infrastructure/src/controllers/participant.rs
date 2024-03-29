use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use command_repository::user_account::participant::ParticipantUserRepository;
use domain::model::{
    condition::Condition,
    gender::{gender_from_i8, Gender},
    region::Region,
    target_status::TargetStatus,
    terms::Terms,
    theme::Theme,
    user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
};

use super::{AppData, WriteApiResponseFailureBody, WriteApiResponseSuccessBody};

/// 参加者アカウントの作成時のリクエストボディを表す構造体
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
    #[schema(required = true, value_type = String, example = "2002-06-22")]
    pub birthday: NaiveDate,
    #[schema(required = true)]
    pub profile: String,
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
    #[schema(required = true)]
    pub target_status: String,
}

/// 参加者アカウントの更新時のリクエストボディを表す構造体
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
    #[schema(required = true, value_type = String, example = "2002-06-22")]
    pub birthday: NaiveDate,
    #[schema(required = true)]
    pub profile: String,
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
    #[schema(required = true)]
    pub target_status: String,
}

/// 参加者アカウントの削除時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteParticipantAccountRequestBody {
    #[schema(required = true)]
    pub pid: String,
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
pub async fn create_participant_account(
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

    let profile: String = body.profile;

    let birthday: NaiveDate = body.birthday;

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
        .map(|c: &String| Condition::from_str(c).unwrap())
        .collect::<Vec<Condition>>();
    let required_condition: Vec<Condition> = body
        .required_condition
        .iter()
        .map(|c: &String| Condition::from_str(c).unwrap())
        .collect::<Vec<Condition>>();
    let target_status: TargetStatus = match TargetStatus::from_str(&body.target_status) {
        Ok(target_status) => target_status,
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

    let terms: Terms = Terms::new(
        region,
        theme,
        required_theme,
        condition,
        required_condition,
        vec![target_status],
    );

    match repository
        .create(pid, name, furigana, phone, gender, birthday, profile, terms)
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
pub async fn update_participant_account(
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

    let profile: String = body.profile;
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
        .map(|c: &String| Condition::from_str(c).unwrap())
        .collect::<Vec<Condition>>();
    let required_condition: Vec<Condition> = body
        .required_condition
        .iter()
        .map(|c: &String| Condition::from_str(c).unwrap())
        .collect::<Vec<Condition>>();
    let target_status: TargetStatus = match TargetStatus::from_str(&body.target_status) {
        Ok(target_status) => target_status,
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

    let terms: Terms = Terms::new(
        region,
        theme,
        required_theme,
        condition,
        required_condition,
        vec![target_status],
    );

    match repository
        .update(pid, name, furigana, phone, gender, birthday, profile, terms)
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
pub async fn delete_participant_account(
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
