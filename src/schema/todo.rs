use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
pub struct Todo {
    /// returns the todo text from db
    text: String,
    completed: bool,
    id: u8,
}

#[derive(Default)]
pub(super) struct TodosQuery {}

#[Object]
impl TodosQuery {
    /// returns the current todos items from the db
    async fn todos(&self) -> Vec<Todo> {
        vec![Todo {
            text: String::from("go to bid"),
            completed: false,
            id: 10,
        }]
    }
}
