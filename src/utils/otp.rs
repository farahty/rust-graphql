use crate::db::GraphQLResult;
use async_graphql::Error;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::prelude::*;

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

pub(crate) fn hash_otp(otp: String) -> GraphQLResult<String> {
    hash(otp, DEFAULT_COST).map_err(|_| Error::new("failed to hash otp"))
}

pub(crate) fn generate_otp() -> String {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(1000..9999);

    format!("{}", x)
}
