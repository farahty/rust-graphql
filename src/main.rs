mod db;
mod models;
mod resolvers;
mod services;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::GraphQL;
use dotenv::dotenv;

use futures::TryStreamExt;
use models::Brand;
use mongodb::Database;
use poem::listener::TcpListener;
use poem::web::{Data, Html, Json};
use poem::{get, handler, middleware::AddData, EndpointExt, IntoResponse, Route, Server};

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[handler]
async fn get_brands(db: Data<&Database>) -> Json<serde_json::Value> {
    let brands = db
        .collection("brands")
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<Brand>>()
        .await
        .unwrap();

    Json(serde_json::json!(brands))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let database = db::connect().await;
    let schema = resolvers::build_schema().data(database.clone()).finish();
    let app = Route::new()
        .at("/", get(graphql_playground).post(GraphQL::new(schema)))
        .at("/brands", get(get_brands))
        .with(AddData::new(database));

    let server = Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)));

    println!("🚀 Server start at http://localhost:{}/", port);

    if let Err(error) = server.run(app).await {
        println!(
            "❌ Failed to start server at http://localhost:{}/ \n {:?}",
            port, error
        )
    }
}
