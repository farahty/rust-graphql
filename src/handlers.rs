use crate::resolvers::TokenSchema;
use crate::utils::get_user_from_header;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::http::HeaderMap;
use poem::web::{Data, Html};
use poem::{handler, IntoResponse};

#[handler]
pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[handler]
pub async fn graphql_endpoint(
    schema: Data<&TokenSchema>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;
    match get_user_from_header(headers) {
        Ok(user) => {
            req = req.data(user);
        }
        Err(_) => {}
    }
    schema.execute(req).await.into()
}
