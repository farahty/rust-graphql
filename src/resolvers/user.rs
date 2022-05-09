use async_graphql::{Context, Error, Object};
use mongodb::bson::doc;

use crate::{
    db::{self, GraphQLResult},
    models::{
        CheckPassword, CreateUserInput, HashPassword, LoginResponse, OTPLoginInput,
        PasswordLoginInput, User, VerifyOTPLoginInput,
    },
};

const COLLECTION: &str = "users";

#[derive(Default)]
pub(super) struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn users(&self, ctx: &Context<'_>) -> GraphQLResult<Vec<User>> {
        db::find(ctx, COLLECTION).await
    }

    async fn user(&self, ctx: &Context<'_>, id: String) -> GraphQLResult<Option<User>> {
        db::find_by_id(ctx, COLLECTION, id).await
    }
}

#[derive(Default)]
pub(super) struct UsersMutation;

#[Object]
impl UsersMutation {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        mut user: CreateUserInput,
    ) -> GraphQLResult<Option<User>> {
        user.hash_password()?;
        db::create(ctx, COLLECTION, user).await
    }

    async fn password_login(
        &self,
        ctx: &Context<'_>,
        input: PasswordLoginInput,
    ) -> GraphQLResult<Option<LoginResponse>> {
        // todo: security check
        let filter = doc! {"$or": [ { "mobile": { "$regex": input.identity.clone() } }, { "email": input.identity } ]};
        let results: Option<User> = db::find_one(ctx, COLLECTION, filter).await?;
        let user = match results {
            Some(u) => u,
            None => return Err(Error::new("user not found")),
        };

        // check user password
        user.check_password(input.password)?;

        // check if the user is active

        // create access token

        // create refresh token

        // create response payload

        Ok(None)
    }

    async fn otp_login(&self, ctx: &Context<'_>, input: OTPLoginInput) -> GraphQLResult<bool> {
        Ok(false)
    }

    async fn verify_otp_login(
        &self,
        ctx: &Context<'_>,
        input: VerifyOTPLoginInput,
    ) -> GraphQLResult<Option<LoginResponse>> {
        Ok(None)
    }
}
