use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::model::user_account::{
    user_id::UserId, user_name::UserName, user_name_furigana::UserNameFurigana,
    user_phone::UserPhone,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantsAccountEvent {
    /// 参加者アカウントが作成された
    ParticipantsAccountCreated(ParticipantsAccountEventCreatedBody),
    /// 参加者アカウントが更新された
    ParticipantsAccountUpdated(ParticipantsAccountEventUpdatedBody),
    /// 参加者アカウントが削除された
    ParticipantsAccountDeleted(ParticipantsAccountEventDeletedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantsAccountEventCreatedBody {
    pub uid: UserId,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub phone: UserPhone,
    pub gender: i8,
    pub birthday: NaiveDate,
    pub region: Vec<String>,
    pub profile: String,
}

impl ParticipantsAccountEventCreatedBody {
    pub fn new(
        uid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: i8,
        birthday: NaiveDate,
        region: Vec<String>,
        profile: String,
    ) -> ParticipantsAccountEventCreatedBody {
        ParticipantsAccountEventCreatedBody {
            uid,
            name,
            furigana,
            phone,
            gender,
            birthday,
            region,
            profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantsAccountEventUpdatedBody {
    pub uid: UserId,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub phone: UserPhone,
    pub gender: i8,
    pub birthday: NaiveDate,
    pub region: Vec<String>,
    pub profile: String,
}

impl ParticipantsAccountEventUpdatedBody {
    pub fn new(
        uid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        gender: i8,
        birthday: NaiveDate,
        region: Vec<String>,
        profile: String,
    ) -> ParticipantsAccountEventUpdatedBody {
        ParticipantsAccountEventUpdatedBody {
            uid,
            name,
            furigana,
            phone,
            gender,
            birthday,
            region,
            profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantsAccountEventDeletedBody {
    pub uid: UserId,
}

impl ParticipantsAccountEventDeletedBody {
    pub fn new(uid: UserId) -> ParticipantsAccountEventDeletedBody {
        ParticipantsAccountEventDeletedBody { uid }
    }
}
