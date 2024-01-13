use serde::{Deserialize, Serialize};

use super::{user_account::{user_name::UserName, user_name_furigana::UserNameFurigana}, gender::Gender, apply::ApplyId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParticipants {
    pub gpid: ApplyId,
    pub serial: u16,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub gender: Gender,
    pub age: u8
}

impl GroupParticipants {
    pub fn new(
        gpid: ApplyId,
        serial: u16,
        name: UserName,
        furigana: UserNameFurigana,
        gender: Gender,
        age: u8
    ) -> GroupParticipants {
        GroupParticipants{
            gpid,
            serial,
            name,
            furigana,
            gender,
            age
        }
    }
}
