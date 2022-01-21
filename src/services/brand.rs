use async_graphql::Context;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::models::Brand;

pub(crate) async fn find(ctx: &Context<'_>) -> Vec<Brand> {
    let mut brands: Vec<Brand> = Vec::new();
    let coll = ctx
        .data::<Database>()
        .unwrap()
        .collection::<Brand>("brands");

    let mut cursor = coll.find(None, None).await.expect("failed to load brands");

    while let Some(brand) = cursor.try_next().await.unwrap() {
        brands.push(brand);
    }

    brands
}

pub(crate) async fn find_by_id(ctx: &Context<'_>, id: String) -> Option<Brand> {
    let coll = ctx
        .data::<Database>()
        .unwrap()
        .collection::<Brand>("brands");

    let oid = ObjectId::parse_str(id).unwrap();

    coll.find_one(doc! {"_id": oid}, None).await.unwrap()
}
