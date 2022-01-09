use mongodb::{Client, Collection, Database};

async fn connect() -> Result<Database, mongodb::error::Error> {
    let db_uri = std::env::var("DB_URI").expect("database uri is not available");
    let db_name = std::env::var("DB_NAME").expect("database name is not available");

    let clint = Client::with_uri_str(&db_uri).await?.database(&db_name);

    Ok(clint)
}

pub async fn collection<T>(model: &str) -> Result<Collection<T>, String> {
    if let Ok(db) = connect().await {
        let coll = db.collection::<T>(model);
        return Ok(coll);
    }

    Err("unable to connect and get the collection".to_string())
}
