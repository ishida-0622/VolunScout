use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use anyhow::Result;
use axum::{headers::HeaderValue, Router};
use config::{Config, File};
use dotenv::dotenv;
use lambda_http::{run, Body, Error};
use serde::Deserialize;
use sqlx::{MySql, MySqlPool, Pool};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use command_infrastructure::controllers::create_router;

use write_api_server::ApiDoc;

#[derive(Deserialize, Debug)]
struct AppSettings {
    api: ApiSettings,
}

#[derive(Deserialize, Debug)]
struct ApiSettings {
    host: String,
    port: u16,
    allow_origins: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // アプリケーション設定を取得
    let app_settings: AppSettings = load_app_config().unwrap();

    // データベース接続URLを環境変数から取得
    let database_url: String = env::var("DATABASE_URL").unwrap();

    // データベースプールを作成し, MySqlに接続
    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await.unwrap();

    // --- AWS Lambda上で動かす場合 ---
    // let app: Router<(), Body> = create_router(pool)
    //     .layer(create_cors_layer(&app_settings))
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run(app).await
    // ---------------------------

    // --- ローカル or EC2上で動かす場合 ---
    let app: Router = create_router(pool)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(create_cors_layer(&app_settings));

    let socket_addr: SocketAddr = SocketAddr::new(
        IpAddr::from_str(&app_settings.api.host).unwrap(),
        app_settings.api.port,
    );

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
    // ---------------------------
}

/// アプリケーション設定を読み込む関数
fn load_app_config() -> Result<AppSettings> {
    // Configオブジェクトを構築し, 設定ファイルを読み込む
    let config: Config = Config::builder()
        .add_source(File::with_name("config/write-api-server").required(false))
        .build()?;

    // 設定ファイルからアプリケーション設定をデシリアライズ
    let app_config: AppSettings = config.try_deserialize()?;

    // デシリアライズしたアプリケーション設定を返す
    Ok(app_config)
}

/// CORS (Cross-Origin Resource Sharing) レイヤーを作成する関数
fn create_cors_layer(app_settings: &AppSettings) -> CorsLayer {
    // 設定から許可するオリジンを取得し, それをHeaderValueに変換
    let allow_origins: Vec<HeaderValue> = app_settings
        .api
        .allow_origins
        .iter()
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<_>>();

    // CORSレイヤーを作成し, 設定に基づいて許可されたオリジン, ヘッダー, メソッドを設定
    CorsLayer::new()
        // TODO: なんかうまくいかないからAnyにしてる
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any)
}
