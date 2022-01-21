mod db;
mod models;
mod resolvers;
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

    let client = db::connect()
        .await
        .expect("failed to connect to database ...");

    let schema = resolvers::build_schema().data(client.clone()).finish();
    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap()
}
