//use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    //    /// Json error.
//    /// #[fail(display = "JSON error: {}", _0)]
//    Json(String),
//
//    /// #[fail(display = "Rpc error: {}", _0)]
//    Rpc(String),
//
//    /// #[fail(display = "Subscription error: {}", _0)]
//    Subscribe(String),
//
//    /// #[fail(display = "Submit error: {}", _0)]
//    Submit(String),
}

#[derive(Debug)]
pub struct Error {
    inner: ErrorKind,
}
pub type Result<T> = std::result::Result<T, Error>;
