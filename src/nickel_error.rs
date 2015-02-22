use std::borrow::{IntoCow, Cow};
use hyper::status::StatusCode;
use std::error::FromError;
use std::old_io::IoError;
use response::Response;
use hyper::net::Streaming;

pub use self::NickelErrorKind::{ErrorWithStatusCode, UserDefinedError, Other};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.

pub struct NickelError<'a, 'b: 'a> {
    pub stream: Option<Response<'a, 'b, Streaming>>,
    pub kind: NickelErrorKind,
    pub message: Cow<'static, str>
}

impl<'a, 'b> NickelError<'a, 'b> {
    /// Creates a new `NickelError` instance
    ///
    /// # Example
    /// ```{rust,ignore}
    /// NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest));
    /// ```
    pub fn new<T>(stream: Option<Response<'a, 'b, Streaming>>,
                  message: T,
                  kind: NickelErrorKind) -> NickelError<'a, 'b>
            where T: IntoCow<'static, str> {
        NickelError {
            stream: stream,
            message: message.into_cow(),
            kind: kind
        }
    }
}

impl<'a, 'b> FromError<IoError> for NickelError<'a, 'b> {
    fn from_error(err: IoError) -> NickelError<'a, 'b> {
        NickelError::new(None, err.desc, ErrorWithStatusCode(StatusCode::InternalServerError))
    }
}

#[derive(Debug)]
pub enum NickelErrorKind {
    // FIXME: Should probably re-export hyper::status::StatusCode
    ErrorWithStatusCode(StatusCode),
    UserDefinedError(usize, String),
    Other
}
