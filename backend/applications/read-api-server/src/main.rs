use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;
use axum::{headers::HeaderValue, Router};
use dotenv::dotenv;
use sqlx::{MySql, MySqlPool, Pool};
use tower_http::cors::{Any, CorsLayer};

use query_infrastructure::controllers::create_router;
use read_api_server::{load_app_config, AppSettings};

#[tokio::main]
async fn main() -> Result<()> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // アプリケーション設定を取得
    let app_settings: AppSettings = load_app_config().unwrap();

    // データベース接続URLを環境変数から取得
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // データベースプールを作成し, MySqlに接続
    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await?;

    // アプリケーションのルーターを作成
    let router: Router = create_router(pool).layer(create_cors_layer(&app_settings));

    // サーバーのアドレスを指定
    let socket_addr: SocketAddr = SocketAddr::new(
        IpAddr::from_str(&app_settings.api.host).unwrap(),
        app_settings.api.port,
    );

    // サーバーを起動
    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

/// CORS (Cross-Origin Resource Sharing) レイヤーを作成する関数
fn create_cors_layer(app_settings: &AppSettings) -> CorsLayer {
    // 設定から許可するオリジンを取得し、それをHeaderValueに変換
    let allow_origins: Vec<HeaderValue> = app_settings
        .api
        .allow_origins
        .iter()
        .map(|origin| origin.parse().unwrap())
        .collect::<Vec<_>>();

    // CORSレイヤーを作成し、設定に基づいて許可されたオリジン、ヘッダー、メソッドを設定
    CorsLayer::new()
        .allow_origin(allow_origins)
        .allow_headers(Any)
        .allow_methods(Any)
}
