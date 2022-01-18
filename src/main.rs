mod db;
mod models;
mod schema;
mod services;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::GraphQL;
use dotenv::dotenv;

use poem::listener::TcpListener;
use poem::web::Html;
use poem::{get, handler, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let schema = schema::build_schema();
    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));
    services::brand::find().await;
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap()
}
