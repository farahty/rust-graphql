use async_graphql::{Context, Error, Object};
use mongodb::bson::doc;

use crate::{
    db::{self, GraphQLResult},
    models::{
        CreateUserInput, LoginResponse, OTPLoginInput, PasswordLoginInput, User, UserStatus,
        VerifyOTPLoginInput,
    },
    utils::{generate_otp, hash_otp, CheckOTP, CheckPassword, HashPassword},
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
        db::create(ctx, COLLECTION, user.try_into()?).await
    }

    async fn password_login(
        &self,
        ctx: &Context<'_>,
        input: PasswordLoginInput,
    ) -> GraphQLResult<Option<LoginResponse>> {
        // todo: security check , the identity should be valid email or mobile number
        let filter = doc! {"$or": [ { "mobile": { "$regex": input.identity.clone() } }, { "email": input.identity } ]};
        let results: Option<User> = db::find_one(ctx, COLLECTION, filter).await?;
        let user = match results {
            Some(u) => u,
            None => return Err(Error::new("user not found")),
        };

        // check user password
        user.check_password(input.password)?;

        // check if the user is active

        match user.status {
            UserStatus::Active => {}
            UserStatus::Expired => return Err(Error::new("user expired")),
            UserStatus::Blocked => return Err(Error::new("user blocked")),
            UserStatus::Suspended => return Err(Error::new("user Suspended")),
        }

        // create access token

        let access_token = user.generate_access_token()?;

        // create refresh token

        let refresh_token = user.generate_refresh_token()?;

        // create response payload

        let response = LoginResponse {
            access_token,
            refresh_token,
            user: user.get_jwt_user(),
        };

        Ok(Some(response))
    }

    async fn otp_login(&self, ctx: &Context<'_>, input: OTPLoginInput) -> GraphQLResult<bool> {
        // create filter object to find if there is an already user
        // todo: security check , the identity should be valid email or mobile number
        let filter = doc! { "mobile": { "$regex": input.mobile.clone() } };

        // find the user on db

        let results: Option<User> = db::find_one(ctx, COLLECTION, filter).await?;
        let user = match results {
            Some(u) => u,
            // create new user on db if there is no user
            None => {
                let results: Option<User> =
                    db::create(ctx, COLLECTION, doc! {"mobile": input.mobile}).await?;
                match results {
                    Some(u) => u,
                    None => return Err(Error::new("failed to create temporary user")),
                }
            }
        };

        // generate and hash new otp code

        let otp = generate_otp();
        let hashed_otp = hash_otp(otp.clone())?;

        // send sms to user mobile with otp code
        dbg!(otp);

        // save the hash on db

        if let Some(user_id) = user.id {
            let doc = doc! {"otp_hash": hashed_otp};
            let _: Option<User> = db::update_by_id(ctx, COLLECTION, user_id, doc).await?;

            return Ok(true);
        }

        Ok(false)
    }

    async fn verify_otp_login(
        &self,
        ctx: &Context<'_>,
        input: VerifyOTPLoginInput,
    ) -> GraphQLResult<Option<LoginResponse>> {
        // todo: security check , the identity should be valid email or mobile number
        let filter = doc! { "mobile": { "$regex": input.mobile.clone() } };
        let results: Option<User> = db::find_one(ctx, COLLECTION, filter).await?;
        let user = match results {
            Some(u) => u,
            None => return Err(Error::new("user not found")),
        };

        // check otp
        user.check_otp(input.otp)?;

        // check if the user is active

        match user.status {
            UserStatus::Active => {}
            UserStatus::Expired => return Err(Error::new("user expired")),
            UserStatus::Blocked => return Err(Error::new("user blocked")),
            UserStatus::Suspended => return Err(Error::new("user Suspended")),
        }

        // create access token

        let access_token = user.generate_access_token()?;

        // create refresh token

        let refresh_token = user.generate_refresh_token()?;

        // create response payload

        let response = LoginResponse {
            access_token,
            refresh_token,
            user: user.get_jwt_user(),
        };

        Ok(Some(response))
    }
}
