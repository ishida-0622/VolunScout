use serde::{Deserialize, Serialize};

use crate::model::user_account::{
    user_name::UserName, user_name_furigana::UserNameFurigana, user_phone::UserPhone, UserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupAccountEvent {
    /// 団体アカウントが作成された
    GroupAccountCreated(GroupAccountEventCreatedBody),
    /// 団体アカウントが更新された
    GroupAccountUpdated(GroupAccountEventUpdatedBody),
    /// 団体アカウントが削除された
    GroupAccountDeleted(GroupAccountEventDeletedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAccountEventCreatedBody {
    pub uid: UserId,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub phone: UserPhone,
    pub contents: String,
}

impl GroupAccountEventCreatedBody {
    pub fn new(
        uid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        contents: String,
    ) -> GroupAccountEventCreatedBody {
        GroupAccountEventCreatedBody {
            uid,
            name,
            furigana,
            phone,
            contents,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAccountEventUpdatedBody {
    pub uid: UserId,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub phone: UserPhone,
    pub contents: String,
}

impl GroupAccountEventUpdatedBody {
    pub fn new(
        uid: UserId,
        name: UserName,
        furigana: UserNameFurigana,
        phone: UserPhone,
        contents: String,
    ) -> GroupAccountEventUpdatedBody {
        GroupAccountEventUpdatedBody {
            uid,
            name,
            furigana,
            phone,
            contents,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupAccountEventDeletedBody {
    pub uid: UserId,
}

impl GroupAccountEventDeletedBody {
    pub fn new(uid: UserId) -> GroupAccountEventDeletedBody {
        GroupAccountEventDeletedBody { uid }
    }
}
