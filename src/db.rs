use async_graphql::*;
use chrono::Utc;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client, Database,
};
use serde::de::DeserializeOwned;

use crate::models::JwtUser;

pub type GraphQLResult<T> = std::result::Result<T, Error>;

pub async fn connect() -> Database {
    let db_uri = std::env::var("DB_URI").expect("database uri is not available");
    let db_name = std::env::var("DB_NAME").expect("database name is not available");

    Client::with_uri_str(&db_uri)
        .await
        .expect("failed to connect to database")
        .database(&db_name)
}

pub(crate) async fn find<T: DeserializeOwned + Unpin + Sync + std::marker::Send>(
    ctx: &Context<'_>,
    collection: &str,
) -> GraphQLResult<Vec<T>> {
    let data = ctx
        .data::<Database>()?
        .collection::<T>(collection)
        .find(None, None)
        .await?
        .try_collect::<Vec<T>>()
        .await?;

    Ok(data)
}

pub(crate) async fn find_by_id<T: DeserializeOwned + Unpin + Sync + Send>(
    ctx: &Context<'_>,
    collection: &str,
    id: String,
) -> GraphQLResult<Option<T>> {
    let coll = ctx.data::<Database>()?.collection::<T>(collection);

    let oid = ObjectId::parse_str(id)?;

    Ok(coll.find_one(doc! {"_id": oid}, None).await?)
}

pub(crate) async fn find_one<T: DeserializeOwned + Unpin + Sync + Send>(
    ctx: &Context<'_>,
    collection: &str,
    filter: impl Into<Option<Document>>,
) -> GraphQLResult<Option<T>> {
    let coll = ctx.data::<Database>()?.collection::<T>(collection);

    Ok(coll.find_one(filter, None).await?)
}

pub(crate) async fn create<T: DeserializeOwned + Unpin + Sync + Send>(
    ctx: &Context<'_>,
    collection: &str,
    mut doc: Document,
) -> GraphQLResult<Option<T>> {
    let database = ctx.data::<Database>()?;

    doc.extend(doc! {"created_at": Utc::now()});

    if let Ok(user) = ctx.data::<JwtUser>() {
        doc.extend(doc! { "created_by": user.id, "owner": user.id });
    }

    let results = database
        .collection(collection)
        .insert_one(doc, None)
        .await?;

    Ok(database
        .collection::<T>(collection)
        .find_one(doc! {"_id": results.inserted_id}, None)
        .await?)
}

pub(crate) async fn update_by_id<T: DeserializeOwned + Unpin + Sync + Send>(
    ctx: &Context<'_>,
    collection: &str,
    id: ObjectId,
    mut doc: Document,
) -> GraphQLResult<Option<T>> {
    let database = ctx.data::<Database>()?;

    doc.extend(doc! { "updated_at": Utc::now() });

    if let Ok(user) = ctx.data::<JwtUser>() {
        doc.extend(doc! { "updated_by": user.id });
    }

    let results = database
        .collection::<T>(collection)
        .update_one(doc! {"_id": id}, doc! {"$set": doc}, None)
        .await?;

    Ok(database
        .collection::<T>(collection)
        .find_one(doc! {"_id": results.upserted_id}, None)
        .await?)
}
