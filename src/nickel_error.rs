use std::borrow::{IntoCow, Cow};
use hyper::status::StatusCode;
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
    /// # extern crate nickel;
    ///
    /// # fn main() {
    /// use nickel::{Request, Response, MiddlewareResult, Halt, MediaType, get_media_type};
    /// use nickel::{NickelError, ErrorWithStatusCode};
    /// use nickel::status::StatusCode;
    ///
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     let stream = try!(res.start());
    ///     Err(NickelError::new(stream,
    ///                          "Error Parsing JSON",
    ///                          ErrorWithStatusCode(StatusCode::BadRequest)))
    /// }
    /// # }
    /// ```
    pub fn new<T>(stream: Response<'a, Streaming>,
                  message: T,
                  kind: NickelErrorKind) -> NickelError<'a>
            where T: IntoCow<'static, str> {
        NickelError {
            stream: Some(stream),
            message: message.into_cow(),
            kind: kind
        }
    }

    /// Creates a new `NickelError` without a `Response`.
    ///
    /// This should only be called in a state where the `Response` has
    /// has failed in an unrecoverable state. If there is an available
    /// `Response` then it must be provided to `new` so that the
    /// underlying stream can be flushed, allowing future requests.
    ///
    /// This is considered `unsafe` as deadlock can occur if the `Response`
    /// does not have the underlying stream flushed when processing is finished.
    pub unsafe fn without_response<T>(message: T,
                                      kind: NickelErrorKind) -> NickelError<'a>
            where T: IntoCow<'static, str> {
        NickelError {
            stream: None,
            message: message.into_cow(),
            kind: kind
        }
    }

    pub fn end(self) -> Option<io::Result<()>> {
        self.stream.map(|s| s.end())
    }
}

#[derive(Debug)]
pub enum NickelErrorKind {
    ErrorWithStatusCode(StatusCode),
    UserDefinedError(usize, String),
    Other
}
