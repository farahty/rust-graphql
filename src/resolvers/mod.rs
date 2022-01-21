pub mod brand;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Default, MergedObject)]
pub struct Query(brand::BrandsQuery);

pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
