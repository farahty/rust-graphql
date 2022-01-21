use async_graphql::Context;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};

use crate::models::Brand;

pub(crate) async fn find(ctx: &Context<'_>) -> Vec<Brand> {
    ctx.data::<Database>()
        .unwrap()
        .collection::<Brand>("brands")
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<Brand>>()
        .await
        .unwrap()
}

pub(crate) async fn find_by_id(ctx: &Context<'_>, id: String) -> Option<Brand> {
    let coll = ctx
        .data::<Database>()
        .unwrap()
        .collection::<Brand>("brands");

    let oid = ObjectId::parse_str(id).unwrap();

    coll.find_one(doc! {"_id": oid}, None).await.unwrap()
}
