use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum Status {
    Draft,
    Published,
    Trashed,
    Deleted,
    Scheduled,
    Expired,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct Translated {
    pub value: String,
    pub language: Option<String>,

    #[serde(rename = "isPrimary")]
    pub is_primary: Option<bool>,
}
