use async_graphql::SimpleObject;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct Brand {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub slug: String,
}
