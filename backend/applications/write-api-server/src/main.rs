use axum::Router;
use dotenv::dotenv;
use lambda_http::{run, Body, Error};
use sqlx::{MySql, MySqlPool, Pool};
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use write_api_server::ApiDoc;

use command_infrastructure::controllers::create_router;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").unwrap();

    let pool: Pool<MySql> = MySqlPool::connect(&database_url).await.unwrap();

    let app: Router<(), Body> = create_router(pool)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    run(app).await
}
