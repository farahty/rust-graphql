use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "_id")]
    id: ObjectId,

    slug: String,
}
