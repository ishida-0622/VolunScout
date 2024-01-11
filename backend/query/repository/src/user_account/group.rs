use anyhow::Result;
use async_graphql::SimpleObject;
use async_trait::async_trait;
use chrono::NaiveDateTime;

use domain::model::user_account::user_id::UserId;

use crate::MySqlBool;

// Read server で返す型. GraphQLのスキーマに対応する
/// 団体アカウントリードモデル
#[derive(SimpleObject)]
pub struct GroupAccount {
    /// 団体アカウントid
    pub gid: String,
    /// 団体名
    pub name: String,
    /// 団体名(フリガナ)
    pub furigana: String,
    /// 電話番号
    pub phone: String,
    /// 住所
    pub address: String,
    /// 団体紹介
    pub contents: String,
    // 代表者氏名
    pub representative_name: String,
    // 代表者氏名(フリガナ)
    pub representative_furigana: String,
    /// 有料会員
    pub is_paid: MySqlBool,
    /// 削除フラグ
    pub is_deleted: MySqlBool,
    /// 削除日時
    pub deleted_at: Option<NaiveDateTime>,
}

impl GroupAccount {
    pub fn new(
        gid: String,
        name: String,
        furigana: String,
        phone: String,
        address: String,
        contents: String,
        representative_name: String,
        representative_furigana: String,
        is_paid: MySqlBool,
        is_deleted: MySqlBool,
        deleted_at: Option<NaiveDateTime>,
    ) -> GroupAccount {
        GroupAccount {
            gid,
            name,
            furigana,
            phone,
            address,
            contents,
            representative_name,
            representative_furigana,
            is_paid,
            is_deleted,
            deleted_at,
        }
    }
}

#[async_trait]
pub trait GroupUserRepository: Send + Sync {
    /// 団体アカウントをIDで取得する
    async fn find_by_id(&self, gid: &UserId) -> Result<GroupAccount>;

    /// 団体アカウントをIDで複数取得する
    async fn find_by_ids(&self, gids: &[UserId]) -> Result<Vec<GroupAccount>>;

    /// 団体アカウントを全て取得する
    async fn find_all(&self) -> Result<Vec<GroupAccount>>;
}
