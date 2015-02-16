use std::borrow::{IntoCow, Cow};
use hyper::status::StatusCode;
use std::error::FromError;
use std::old_io::IoError;

pub use self::NickelErrorKind::{ErrorWithStatusCode, UserDefinedError, Other};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.

#[derive(Debug)]
pub struct NickelError {
    pub kind: NickelErrorKind,
    pub message: Cow<'static, str>
}

impl NickelError {
    /// Creates a new `NickelError` instance
    ///
    /// # Example
    /// ```{rust,ignore}
    /// NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest));
    /// ```
    pub fn new<T: IntoCow<'static, str>>(message: T, kind: NickelErrorKind) -> NickelError {
        NickelError {
            message: message.into_cow(),
            kind: kind
        }
    }
}

impl FromError<IoError> for NickelError {
    fn from_error(err: IoError) -> NickelError {
        NickelError::new(err.desc, ErrorWithStatusCode(StatusCode::InternalServerError))
    }
}

#[derive(Debug)]
pub enum NickelErrorKind {
    // FIXME: Should probably re-export hyper::status::StatusCode
    ErrorWithStatusCode(StatusCode),
    UserDefinedError(usize, String),
    Other
}
