use async_graphql::*;

use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
use parse_duration::parse;
use serde::{Deserialize, Serialize};

use crate::{
    db::GraphQLResult,
    utils::{CheckPassword, HashPassword},
};

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
    pub token: Option<String>,

    #[serde(default)]
    pub role: Role,

    #[serde(default)]
    pub verified: bool,

    #[serde(default)]
    pub status: UserStatus,
}

impl CheckPassword for User {
    fn get_hashed_password(&self) -> Option<String> {
        self.password.clone()
    }
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct JwtUser {
    pub id: ObjectId,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub role: Role,
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: JwtUser,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum Role {
    User,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub(crate) enum UserStatus {
    Active,
    Expired,
    Blocked,
    Suspended,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub(crate) struct CreateUserInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
    pub role: Role,
}

impl HashPassword for CreateUserInput {
    fn get_password(&self) -> Option<String> {
        self.password.clone()
    }

    fn set_password(&mut self, password: String) {
        self.password = Some(password)
    }
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub(crate) struct SignupInput {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub(crate) struct PasswordLoginInput {
    pub identity: String,
    pub password: String,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub(crate) struct OTPLoginInput {
    pub mobile: Option<String>,
}

#[derive(InputObject, Debug, Serialize, Deserialize)]
pub(crate) struct VerifyOTPLoginInput {
    pub mobile: Option<String>,
    pub otp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct JwtClaims {
    pub sub: String,
    exp: u64,
    user: JwtUser,
}

impl User {
    pub fn get_jwt_user(&self) -> JwtUser {
        let id = match self.id {
            Some(id) => id,
            None => ObjectId::default(),
        };

        JwtUser {
            email: self.email.clone(),
            id,
            mobile: self.mobile.clone(),
            role: self.role,
        }
    }

    pub fn generate_access_token(&self) -> GraphQLResult<String> {
        let duration_str = std::env::var("ACCESS_TOKEN_EXPIRY")
            .map_err(|_| Error::new("failed to load access token expiry form env"))?;

        let duration = parse(duration_str.as_str())
            .map_err(|_| Error::new("failed to parse access token expiry string"))?;

        let secret = std::env::var("ACCESS_TOKEN_SECRET")
            .map_err(|_| Error::new("failed to load access token secret from env"))?;

        let exp = jsonwebtoken::get_current_timestamp() + duration.as_secs();

        let user = self.get_jwt_user();
        let sub = user.id.to_hex();

        let claims = JwtClaims { sub, exp, user };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn generate_refresh_token(&self) -> GraphQLResult<String> {
        let duration_str = std::env::var("REFRESH_TOKEN_EXPIRY")
            .map_err(|_| Error::new("failed to load refresh token expiry form env"))?;

        let duration = parse(duration_str.as_str())
            .map_err(|_| Error::new("failed to parse refresh token expiry string"))?;

        let secret = std::env::var("REFRESH_TOKEN_SECRET")
            .map_err(|_| Error::new("failed to load refresh token secret from env"))?;

        let exp = jsonwebtoken::get_current_timestamp() + duration.as_secs();

        let user = self.get_jwt_user();
        let sub = user.id.to_hex();

        let claims = JwtClaims { sub, exp, user };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(token)
    }
}
