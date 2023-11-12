use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use anyhow::Result;
use dotenv::dotenv;
use sqlx::MySqlPool;

use infra::controllers::read_server::create_router;

#[tokio::main]
async fn main() -> Result<()> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // データベース接続URLを取得
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // データベースプールを作成
    let pool = MySqlPool::connect(&database_url).await?;

    let router = create_router(pool);

    let socket_addr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 18082);

    axum::Server::bind(&socket_addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
