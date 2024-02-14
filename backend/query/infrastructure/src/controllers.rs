use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension, http::StatusCode, response::{self, IntoResponse, Response}, routing::{get, get_service}, Router, body::Body
};
use sqlx::MySqlPool;
use tower_http::services::ServeDir;

use crate::resolvers::{create_schema, ApiSchema};

/// GraphQLのリクエストを受け付けるエンドポイント
async fn graphql_handler(schema: Extension<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// GraphQL IDEのためのエンドポイント
async fn graphql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint(Endpoints::GraphQL.as_str())
            .finish(),
    )
}

pub enum Endpoints {
    GraphQL,
    Assets,
}

impl Endpoints {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Endpoints::GraphQL => "/graphql",
            Endpoints::Assets => "/assets",
        }
    }
}

/// [Router]を生成する関数
pub fn create_router(pool: MySqlPool) -> Router {
    let schema = create_schema(pool);
    let serve_dir = ServeDir::new(&Endpoints::Assets.as_str()[1..]);
    let service = get_service(serve_dir);

    Router::new()
        .route(
            Endpoints::GraphQL.as_str(),
            get(graphql).post(graphql_handler),
        ).route(
            "/health",
            get(|| async {
            Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .unwrap()
        })
        )
        .nest_service(Endpoints::Assets.as_str(), service)
        .layer(Extension(schema))
}
