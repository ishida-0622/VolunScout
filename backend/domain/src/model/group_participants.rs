use serde::{Deserialize, Serialize};
use ulid_generator_rs::{ULIDGenerator, ULID};

use super::{user_account::{user_name::UserName, user_name_furigana::UserNameFurigana}, gender::Gender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParticipants {
    pub gpid: GroupParticipantsId,
    pub serial: u16,
    pub name: UserName,
    pub furigana: UserNameFurigana,
    pub gender: Gender,
    pub age: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupParticipantsId(pub ULID);

impl GroupParticipantsId {
    pub fn new() -> GroupParticipantsId {
        let mut generator: ULIDGenerator = ULIDGenerator::new();
        let value: ULID = generator.generate().unwrap();
        GroupParticipantsId(value)
    }
}

impl GroupParticipants {
    pub fn new(
        gpid: GroupParticipantsId,
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

// Displayを実装することで, to_string()で文字列に変換できるようになる
impl std::fmt::Display for GroupParticipantsId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
