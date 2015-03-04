use std::borrow::{IntoCow, Cow};
use hyper::status::StatusCode;
use std::error::FromError;
use std::io;
use response::Response;
use hyper::net::Streaming;

pub use self::NickelErrorKind::{ErrorWithStatusCode, UserDefinedError, Other};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<'a> {
    pub stream: Option<Response<'a, Streaming>>,
    pub kind: NickelErrorKind,
    pub message: Cow<'static, str>
}

impl<'a> NickelError<'a> {
    /// Creates a new `NickelError` instance
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{NickelError, ErrorWithStatusCode};
    /// use nickel::status::StatusCode;
    /// NickelError::new(None,
    ///                  "Error Parsing JSON",
    ///                  ErrorWithStatusCode(StatusCode::BadRequest));
    /// ```
    pub fn new<T>(stream: Option<Response<'a, Streaming>>,
                  message: T,
                  kind: NickelErrorKind) -> NickelError<'a>
            where T: IntoCow<'static, str> {
        NickelError {
            stream: stream,
            message: message.into_cow(),
            kind: kind
        }
    }

    pub fn end(self) -> Option<io::Result<()>> {
        self.stream.map(|s| s.end())
    }
}

impl<'a> FromError<io::Error> for NickelError<'a> {
    fn from_error(err: io::Error) -> NickelError<'a> {
        NickelError::new(None,
                         err.description().to_string(),
                         ErrorWithStatusCode(StatusCode::InternalServerError))
    }
}

#[derive(Debug)]
pub enum NickelErrorKind {
    ErrorWithStatusCode(StatusCode),
    UserDefinedError(usize, String),
    Other
}
