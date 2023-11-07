use async_graphql::SimpleObject;

pub mod scout;
pub mod user_account;
pub mod volunteer;

/// MySQLで使用するbool型
///
/// MySQLのbool型はi8で表現されるため, i8からbool型に変換するトレイトが必要
#[derive(SimpleObject)]
pub struct MySqlBool {
    value: bool,
}

impl From<i8> for MySqlBool {
    fn from(i: i8) -> Self {
        MySqlBool { value: i != 0 }
    }
}
