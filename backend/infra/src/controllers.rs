use std::str::FromStr;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::NaiveDate;
use domain::{
    model::user_account::{
        user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
        user_phone::UserPhone,
    },
    repository::user_account::GroupUserRepository,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use utoipa::{openapi::schema, ToSchema};

use crate::user_account::group::GroupAccountImpl;

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
    pub gid: String,
    pub name: String,
    pub furigana: String,
    pub phone: String,
    pub address: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteGroupAccountRequestBody {
    pub gid: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateParticipantAccountRequestBody {
    pub pid: String,
    pub name: String,
    pub furigana: String,
    pub phone: String,
    pub gender: u8,
    pub birthday: NaiveDate,
    pub region: Vec<String>,
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateParticipantAccountRequestBody {
    pub pid: String,
    pub name: String,
    pub furigana: String,
    pub phone: String,
    pub gender: u8,
    pub birthday: NaiveDate,
    pub region: Vec<String>,
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteParticipantAccountRequestBody {
    pub pid: String,
}

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
    Json(body): Json<CreateGroupAccountRequestBody>,
) -> impl IntoResponse {
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

    let pool: sqlx::Pool<sqlx::MySql> = MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let repository: GroupAccountImpl = GroupAccountImpl::new(pool);
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
