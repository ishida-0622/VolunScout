use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use command_repository::user_account::group::GroupUserRepository;
use domain::model::user_account::{
    user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
    user_phone::UserPhone,
};

use super::{AppData, WriteApiResponseFailureBody, WriteApiResponseSuccessBody};

/// グループアカウントの作成時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub representative_name: String,
    #[schema(required = true)]
    pub representative_furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub address: String,
    #[schema(required = true)]
    pub contents: String,
    pub photos: Option<Vec<String>>,
}

/// グループアカウントの更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
    #[schema(required = true)]
    pub name: String,
    #[schema(required = true)]
    pub furigana: String,
    #[schema(required = true)]
    pub representative_name: String,
    #[schema(required = true)]
    pub representative_furigana: String,
    #[schema(required = true)]
    pub phone: String,
    #[schema(required = true)]
    pub address: String,
    #[schema(required = true)]
    pub contents: String,
    pub photos: Option<Vec<String>>,
}

/// グループアカウントの削除時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeleteGroupAccountRequestBody {
    #[schema(required = true)]
    pub gid: String,
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

    let representative_name: UserName = match UserName::from_str(&body.representative_name) {
        Ok(representative_name) => representative_name,
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

    let representative_furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.representative_furigana) {
        Ok(representative_furigana) => representative_furigana,
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

    let s3_keys: Vec<String> = match body.photos {
        None => Vec::new(),
        Some(s3_keys) => s3_keys
    };

    match repository
        .create(gid, name, furigana, representative_name, representative_furigana, phone, address, contents, s3_keys)
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

    let representative_name: UserName = match UserName::from_str(&body.representative_name) {
        Ok(representative_name) => representative_name,
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

    let representative_furigana: UserNameFurigana = match UserNameFurigana::from_str(&body.representative_furigana) {
        Ok(representative_furigana) => representative_furigana,
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

    let s3_keys: Vec<String> = match body.photos {
        None => Vec::new(),
        Some(s3_keys) => s3_keys
    };

    match repository
        .update(gid, name, furigana, representative_name, representative_furigana, phone, address, contents, s3_keys)
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
