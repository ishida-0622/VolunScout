use std::{collections::HashMap, str::FromStr};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use command_repository::activities::apply::ApplyRepository;
use domain::model::{
    apply::ApplyId,
    gender::{gender_from_i8, Gender},
    group_participants::GroupParticipants,
    user_account::{user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana},
    volunteer::VolunteerId,
};

use super::{AppData, WriteApiResponseFailureBody, WriteApiResponseSuccessBody};

/// ボランティア応募時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateApplyRequestBody {
    #[schema(required = true)]
    pub vid: String,
    #[schema(required = true)]
    pub uid: String,
    pub members: Option<Vec<HashMap<String, Value>>>,
}

/// 応募承認更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateApplyAllowedStatusRequestBody {
    #[schema(required = true)]
    pub aid: String,
    #[schema(required = true)]
    pub allowed_status: u8,
}

/// 応募メール送信更新時のリクエストボディを表す構造体
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateApplyIsSentRequestBody {
    #[schema(required = true)]
    pub aid: String,
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

    let members: Option<Vec<GroupParticipants>> = if body.members != None {
        let mut return_member: Vec<GroupParticipants> = vec![];
        let m = body.members.unwrap();
        for (ind, mp) in m.iter().enumerate() {
            let body_name = mp.get("name");
            let body_furigana = mp.get("furigana");
            let body_gender = mp.get("gender");
            let body_age = mp.get("age");
            let name: UserName = if body_name == None {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(WriteApiResponseFailureBody {
                        message: "name is none".to_string(),
                    }),
                )
                    .into_response();
            } else {
                let user_name = UserName::from_str(&body_name.unwrap().to_string());
                if let Err(error) = user_name {
                    log::warn!("error = {}", error);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(WriteApiResponseFailureBody {
                            message: error.to_string(),
                        }),
                    )
                        .into_response();
                } else {
                    user_name.unwrap()
                }
            };
            let furigana: UserNameFurigana = if body_furigana == None {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(WriteApiResponseFailureBody {
                        message: "furigana is none".to_string(),
                    }),
                )
                    .into_response();
            } else {
                let user_name_furigana =
                    UserNameFurigana::from_str(&body_furigana.unwrap().to_string());
                if let Err(error) = user_name_furigana {
                    log::warn!("error = {}", error);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(WriteApiResponseFailureBody {
                            message: error.to_string(),
                        }),
                    )
                        .into_response();
                } else {
                    user_name_furigana.unwrap()
                }
            };
            let gender: Gender = if body_gender == None {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(WriteApiResponseFailureBody {
                        message: "gender is none".to_string(),
                    }),
                )
                    .into_response();
            } else {
                let user_gender = gender_from_i8(&(body_gender.unwrap().as_i64().unwrap() as i8));
                if let Err(error) = user_gender {
                    log::warn!("error = {}", error);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(WriteApiResponseFailureBody {
                            message: error.to_string(),
                        }),
                    )
                        .into_response();
                } else {
                    user_gender.unwrap()
                }
            };
            let age: u8 = if body_age == None {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(WriteApiResponseFailureBody {
                        message: "age is none".to_string(),
                    }),
                )
                    .into_response();
            } else {
                let user_age = body_age.unwrap().as_u64();
                if user_age == None {
                    log::warn!("error = {}", "age is not integer".to_string());
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(WriteApiResponseFailureBody {
                            message: "age is not integer".to_string(),
                        }),
                    )
                        .into_response();
                } else {
                    user_age.unwrap() as u8
                }
            };
            let gp = GroupParticipants::new(ind as u16, name, furigana, gender, age);
            return_member.push(gp);
        }
        Some(return_member)
    } else {
        None
    };

    let as_group: bool = match members {
        None => false,
        Some(_) => true,
    };

    match repository.create(aid, vid, uid, as_group, members).await {
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

    match repository.update_allowed_status(aid, allowed_status).await {
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
    request_body=UpdateApplyIsSentRequestBody,
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
