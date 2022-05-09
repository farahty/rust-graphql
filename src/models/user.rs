use async_graphql::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::db::GraphQLResult;

pub(crate) trait HashPassword {
    fn get_password(&self) -> Option<String>;
    fn set_password(&mut self, password: String);

    fn hash_password(&mut self) -> GraphQLResult<bool> {
        if let Some(password) = self.get_password() {
            let hashed = match hash(password, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(_) => return Err(Error::new("failed to hash password")),
            };

            self.set_password(hashed)
        }

        Ok(true)
    }
}

pub(crate) trait CheckPassword {
    fn get_hashed_password(&self) -> Option<String>;

    fn check_password(&self, password: String) -> GraphQLResult<bool> {
        if let Some(hash) = self.get_hashed_password() {
            match verify(password, &hash) {
                Ok(ok) => {
                    if ok {
                        return Ok(true);
                    } else {
                        return Err(Error::new("password not match"));
                    }
                }
                Err(_) => return Err(Error::new("failed to verify the password")),
            }
        }

        Err(Error::new("password not available"))
    }
}

#[derive(SimpleObject, Debug, Serialize, Deserialize)]
pub(crate) struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
    pub token: Option<String>,
    pub role: Role,

    #[serde(default = "bool::default")]
    pub verified: bool,
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
