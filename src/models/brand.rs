use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::schema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Brand {
    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub slug: String,
}

impl From<schema::brand::Brand> for Brand {
    fn from(obj: schema::brand::Brand) -> Self {
        Brand {
            id: ObjectId::from_str(obj.id.as_str()).unwrap(),
            slug: obj.slug,
        }
    }
}

// impl Into<schema::brand::Brand> for Brand {
//     fn into(self) -> schema::brand::Brand {
//         schema::brand::Brand {
//             id: self.id.to_hex(),
//             slug: self.slug,
//         }
//     }
// }
