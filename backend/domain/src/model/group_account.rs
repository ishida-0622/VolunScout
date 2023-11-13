use serde::{Deserialize, Serialize};

use crate::model::user_account::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub user: User,
    pub address: String,
    pub contents: String,
    pub is_paid: bool,
}

impl Group {
    pub fn new(user: User, address: String, contents: String) -> Group {
        Group {
            user,
            address,
            contents,
            is_paid: false,
        }
    }
}
