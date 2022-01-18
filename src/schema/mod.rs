pub mod brand;
mod todo;
mod user;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

#[derive(Default, MergedObject)]
pub struct Query(todo::TodosQuery, user::UserQuery, brand::BrandsQuery);

pub fn build_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
}
