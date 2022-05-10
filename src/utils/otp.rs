use bcrypt::{hash, verify, DEFAULT_COST};

use crate::db::GraphQLResult;
use async_graphql::Error;

pub(crate) trait CheckOTP {
    fn get_hashed_otp(&self) -> Option<String>;

    fn check_otp(&self, otp: String) -> GraphQLResult<bool> {
        if let Some(hash) = self.get_hashed_otp() {
            match verify(otp, &hash) {
                Ok(ok) => {
                    if ok {
                        return Ok(true);
                    } else {
                        return Err(Error::new("otp not match"));
                    }
                }
                Err(_) => return Err(Error::new("failed to verify the otp")),
            }
        }

        Err(Error::new("otp not available"))
    }
}

pub(crate) trait HashOTP {
    fn get_otp(&self) -> Option<String>;
    fn set_otp(&mut self, otp: String);

    fn hash_otp(&mut self) -> GraphQLResult<bool> {
        if let Some(otp) = self.get_otp() {
            let hashed = match hash(otp, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(_) => return Err(Error::new("failed to hash otp")),
            };

            self.set_otp(hashed)
        }

        Ok(true)
    }
}
