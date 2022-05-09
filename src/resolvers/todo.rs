use async_graphql::{Context, Object};

use crate::{
    db::{self, GraphQLResult},
    models::Todo,
};

const COLLECTION: &str = "todos";

#[derive(Default)]
pub(super) struct TodosQuery;

#[Object]
impl TodosQuery {
    async fn todos(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<Todo>> {
        db::find(ctx, COLLECTION).await
    }

    async fn todo(&self, ctx: &Context<'_>, id: String) -> GraphQLResult<Option<Todo>> {
        db::find_by_id(ctx, COLLECTION, id).await
    }
}
