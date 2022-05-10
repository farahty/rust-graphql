use async_graphql::Error;
use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::http::HeaderMap;

use crate::{
    db::GraphQLResult,
    models::{JwtClaims, JwtUser},
};

pub(crate) fn get_user_from_header(headers: &HeaderMap) -> GraphQLResult<JwtUser> {
    let auth_str = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| Error::new("authorization header not available"))?;

    let secret = std::env::var("ACCESS_TOKEN_SECRET")
        .map_err(|_| Error::new("failed to load access token secret from env"))?;

    let token = auth_str
        .split(" ")
        .last()
        .ok_or_else(|| Error::new("authorization header not formatted correctly"))?;

    let jwt = decode::<JwtClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(jwt.claims.user)
}
