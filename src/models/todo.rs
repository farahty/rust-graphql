use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::{Status, Translated};

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub slug: String,
    title: Vec<Translated>,

    body: Option<Vec<Translated>>,

    status: Status,
}
