mod common;
mod todo;
mod user;

pub(crate) use common::Status;
pub(crate) use common::Translated;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub(crate) use todo::Todo;
pub(crate) use user::*;

pub trait Queryable: DeserializeOwned + Unpin + Sync + std::marker::Send + Serialize {}
