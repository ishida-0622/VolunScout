use anyhow::Result;
use config::{Config, File};
use serde::Deserialize;

/// APIの設定を表す構造体
#[derive(Deserialize, Debug)]
pub struct ApiSettings {
    /// APIのホスト名
    pub host: String,
    /// APIのポート番号
    pub port: u16,
    /// 許可するオリジンのリスト
    pub allow_origins: Vec<String>,
}

// アプリケーションの設定を表す構造体
#[derive(Deserialize, Debug)]
pub struct AppSettings {
    pub api: ApiSettings, // アプリケーション内のAPI設定
}

/// アプリケーション設定を読み込む関数
pub fn load_app_config() -> Result<AppSettings> {
    // Configオブジェクトを構築し, 設定ファイルを読み込む
    let config: Config = Config::builder()
        .add_source(File::with_name("config/read-api-server").required(false))
        .build()?;

    // 設定ファイルからアプリケーション設定をデシリアライズ
    let app_config: AppSettings = config.try_deserialize()?;

    Ok(app_config) // デシリアライズしたアプリケーション設定を返す
}
