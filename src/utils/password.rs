use bcrypt::{hash, verify, DEFAULT_COST};

use crate::db::GraphQLResult;
use async_graphql::Error;

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
