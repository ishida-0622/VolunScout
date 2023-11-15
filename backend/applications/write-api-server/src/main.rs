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
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").unwrap();

    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await.unwrap();

    // AWS Lambda上で動かす場合
    // let app: Router<(), Body> = create_router(pool)
    //     .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run(app).await

    // ローカル上で動かす場合
    let app = create_router(pool)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let socket_addr = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), 18081);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
