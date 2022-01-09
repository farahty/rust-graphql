use async_graphql::{Object, SimpleObject};

#[derive(SimpleObject)]
struct User {
    name: String,
    password: String,
}

#[derive(Default)]
pub(super) struct UserQuery {}

#[Object]
impl UserQuery {
    async fn users(&self) -> Vec<User> {
        vec![User {
            name: "the name".to_string(),
            password: "the password".to_string(),
        }]
    }
}
