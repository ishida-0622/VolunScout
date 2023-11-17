use std::{
    env,
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use axum::Router;
use dotenv::dotenv;
use lambda_http::{run, Body, Error};
use sqlx::{MySql, MySqlPool, Pool};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use command_infrastructure::controllers::create_router;

use write_api_server::ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // .envファイルから環境変数をロード
    dotenv().ok();

    // データベース接続URLを環境変数から取得
    let database_url: String = env::var("DATABASE_URL").unwrap();

    // データベースプールを作成し, MySqlに接続
    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await.unwrap();

    // --- AWS Lambda上で動かす場合 ---
    // let app: Router<(), Body> = create_router(pool)
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run(app).await
    // ---------------------------

    // --- ローカル or EC2上で動かす場合 ---
    let app: Router = create_router(pool)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let socket_addr: SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 18081);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
    // ---------------------------
}
