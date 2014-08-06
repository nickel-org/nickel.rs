use std::str::SendStr;
use http::status::Status;

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.

#[deriving(Show)]
pub struct NickelError {
    pub kind: NickelErrorKind,
    pub message: SendStr
}

impl NickelError {
    /// Creates a new `NickelError` instance
    ///
    /// # Example
    /// ```rust
    /// NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest));
    /// ```
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