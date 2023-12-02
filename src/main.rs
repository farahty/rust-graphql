mod db;
mod handlers;
mod models;
mod resolvers;
mod utils;

use crate::handlers::{graphql_endpoint, graphql_playground};
use dotenv::dotenv;
use poem::listener::TcpListener;
use poem::{get, EndpointExt, Route, Server};
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let database = db::connect().await;
    let schema = resolvers::build_schema().data(database.clone()).finish();

    File::create("app.schema.gql")?.write_all(&schema.sdl().as_bytes())?;

    let router = Route::new()
        .at("/", get(graphql_playground).post(graphql_endpoint))
        .data(schema)
        .data(database.clone());

    let server = Server::new(TcpListener::bind(format!("0.0.0.0:{}", port)));

    println!("🚀 Server start at http://localhost:{}/", port);

    if let Err(error) = server.run(router).await {
        println!(
            "❌ Failed to start server at http://localhost:{}/ \n {:?}",
            port, error
        )
    }

    Ok(())
}
