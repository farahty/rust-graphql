use async_graphql::{Context, Object};

use crate::{models::Brand, services};

#[derive(Default)]
pub(super) struct BrandsQuery;

#[Object]
impl BrandsQuery {
    async fn brands(&self, ctx: &Context<'_>) -> Vec<Brand> {
        services::brand::find(ctx).await
    }

    async fn brand(&self, ctx: &Context<'_>, id: String) -> Option<Brand> {
        services::brand::find_by_id(ctx, id).await
    }
}
