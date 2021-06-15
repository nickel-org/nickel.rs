use std::borrow::Cow;
use hyper::StatusCode;
use std::io;
use std::error::Error;
use crate::response::Response;

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<D: Send + 'static + Sync = ()> {
    pub stream: Option<Response<D>>,
    pub message: Cow<'static, str>
}

impl<D: Send + 'static + Sync> NickelError<D> {
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
    /// fn handler<D: Send + 'static + Sync>(_: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
    ///     Err(NickelError::new(res, "Error Parsing JSON", StatusCode::BAD_REQUEST))
    /// }
    /// # }
    /// ```
    pub fn new<T>(mut stream: Response<D>,
                  message: T,
                  status_code: StatusCode) -> NickelError<D>
            where T: Into<Cow<'static, str>> {
        stream.set(status_code);

        stream.start();
        NickelError {
            stream: Some(stream),
            message: message.into(),
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
    pub unsafe fn without_response<T>(message: T) -> NickelError<D>
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

impl<T, D: Send + 'static + Sync> From<(Response<D>, (StatusCode, T))> for NickelError<D>
        where T: Into<Box<dyn Error + 'static>> {
    fn from((res, (errorcode, err)): (Response<D>, (StatusCode, T))) -> NickelError<D> {
        let err = err.into();
        NickelError::new(res, err.to_string(), errorcode)
    }
}

impl<D: Send + 'static + Sync> From<(Response<D>, String)> for NickelError<D> {
    fn from((res, msg): (Response<D>, String)) -> NickelError<D> {
        NickelError::new(res, msg, StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl<D: Send + 'static + Sync> From<(Response<D>, StatusCode)> for NickelError<D> {
    fn from((res, code): (Response<D>, StatusCode)) -> NickelError<D> {
        NickelError::new(res, "", code)
    }
}
