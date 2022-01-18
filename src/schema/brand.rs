use async_graphql::{Object, SimpleObject};

use crate::{models, services};

#[derive(SimpleObject)]
pub struct Brand {
    pub id: String,
    pub slug: String,
}

impl From<models::Brand> for Brand {
    fn from(obj: models::Brand) -> Self {
        Self {
            id: obj.id.to_hex(),
            slug: obj.slug.to_string(),
        }
    }
}

#[derive(Default)]
pub(super) struct BrandsQuery;

#[Object]
impl BrandsQuery {
    async fn brands(&self) -> Vec<Brand> {
        let mut brands: Vec<Brand> = Vec::new();

        let data = services::brand::find().await;

        for item in data {
            brands.push(item.into());
        }

        brands
    }
}
