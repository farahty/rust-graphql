pub mod todo;
pub mod user;

use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(Default, MergedObject)]
pub struct Query(todo::TodosQuery, user::UsersQuery);

#[derive(Default, MergedObject)]
pub struct Mutation(user::UsersMutation);

pub type TokenSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
