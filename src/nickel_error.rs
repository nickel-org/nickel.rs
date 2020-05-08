use std::borrow::Cow;
use hyper::StatusCode;
use std::io;
use std::error::Error;
use crate::response::Response;

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<'a, B, D = ()> {
    pub stream: Option<Response<'a, B, D>>,
    pub message: Cow<'static, str>
}

impl<'a, B, D> NickelError<'a, B, D> {
    /// Creates a new `NickelError` instance.
    ///
    /// You should probably use `Response#error` in favor of this.
    ///
    /// # Examples
    /// ```{rust}
    /// # extern crate nickel;
    ///
    /// # fn main() {
    /// use nickel::{Request, Response, MiddlewareResult, NickelError};
    /// use nickel::status::StatusCode;
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
    ///     Err(NickelError::new(res, "Error Parsing JSON", StatusCode::BadRequest))
    /// }
    /// # }
    /// ```
    pub fn new<T>(mut stream: Response<'a, B, D>,
                  message: T,
                  status_code: StatusCode) -> NickelError<'a, B, D>
            where T: Into<Cow<'static, str>> {
        stream.set(status_code);

        match stream.start() {
            Ok(stream) =>
                NickelError {
                    stream: Some(stream),
                    message: message.into(),
                },
            Err(e) => e
        }
    }

    /// Creates a new `NickelError` without a `Response`.
    ///
    /// This should only be called in a state where the `Response` has
    /// failed in an unrecoverable state. If there is an available
    /// `Response` then it must be provided to `new` so that the
    /// underlying stream can be flushed, allowing future requests.
    ///
    /// This is considered `unsafe` as deadlock can occur if the `Response`
    /// does not have the underlying stream flushed when processing is finished.
    pub unsafe fn without_response<T>(message: T) -> NickelError<'a, B, D>
            where T: Into<Cow<'static, str>> {
        NickelError {
            stream: None,
            message: message.into(),
        }
    }

    pub fn end(self) -> Option<io::Result<()>> {
        self.stream.map(|s| s.end())
    }
}

impl<'a, T, B, D> From<(Response<'a, B, D>, (StatusCode, T))> for NickelError<'a, B, D>
        where T: Into<Box<dyn Error + 'static>> {
    fn from((res, (errorcode, err)): (Response<'a, B, D>, (StatusCode, T))) -> NickelError<'a, B, D> {
        let err = err.into();
        NickelError::new(res, err.to_string(), errorcode)
    }
}

impl<'a, B, D> From<(Response<'a, B, D>, String)> for NickelError<'a, B, D> {
    fn from((res, msg): (Response<'a, B, D>, String)) -> NickelError<'a, B, D> {
        NickelError::new(res, msg, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl<'a, B, D> From<(Response<'a, B, D>, StatusCode)> for NickelError<'a, B, D> {
    fn from((res, code): (Response<'a, B, D>, StatusCode)) -> NickelError<'a, B, D> {
        NickelError::new(res, "", code)
    }
}
