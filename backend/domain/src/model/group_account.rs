use serde::{Deserialize, Serialize};

use crate::model::user_account::User;

use super::user_account::{user_name::UserName, user_name_furigana::UserNameFurigana};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub user: User,
    pub representative_name: UserName,
    pub representative_furigana: UserNameFurigana,
    pub address: String,
    pub contents: String,
    pub is_paid: bool,
}

impl Group {
    pub fn new(
        user: User,
        representative_name: UserName,
        representative_furigana: UserNameFurigana,
        address: String,
        contents: String
    ) -> Group {
        Group {
            user,
            representative_name,
            representative_furigana,
            address,
            contents,
            is_paid: false,
        }
    }
}
