use mongodb::{Client, Database};

pub async fn connect() -> Database {
    let db_uri = std::env::var("DB_URI").expect("database uri is not available");
    let db_name = std::env::var("DB_NAME").expect("database name is not available");

    Client::with_uri_str(&db_uri)
        .await
        .expect("failed to connect to database")
        .database(&db_name)
}
