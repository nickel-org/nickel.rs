use std::str::SendStr;
use http::status::Status;

#[deriving(Show)]
pub struct NickelError {
    kind: NickelErrorKind,
    message: SendStr
}

impl NickelError {
    pub fn new<T: IntoMaybeOwned<'static>>(message: T, kind: NickelErrorKind) -> NickelError {
        NickelError {
            message: message.into_maybe_owned(),
            kind: kind
        }
    }
}

#[deriving(Show)]
pub enum NickelErrorKind {
    ErrorWithStatusCode(Status),
    UserDefinedError(int, String),
    Other
}