use std::borrow::Cow;
use hyper::status::StatusCode;
use std::io;
use std::error::Error;
use response::Response;
use hyper::net::{Fresh, Streaming};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<'a, D: 'a = ()> {
    pub stream: Option<Response<'a, D, Streaming>>,
    pub message: Cow<'static, str>
}

impl<'a, D> NickelError<'a, D> {
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
    pub fn new<T>(mut stream: Response<'a, D, Fresh>,
                  message: T,
                  status_code: StatusCode) -> NickelError<'a, D>
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
    pub unsafe fn without_response<T>(message: T) -> NickelError<'a, D>
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

/// `IntoError` is the required bounds for the `try_with!` macro.
///
/// See the `error_handling` example for usage.
pub trait IntoError<D> : Sized {
    fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D>;
}

impl<D> IntoError<D> for StatusCode {
    fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D> {
        NickelError::new(res, "", self)
    }
}

impl<D> IntoError<D> for String {
    fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D> {
        NickelError::new(res, self, StatusCode::InternalServerError)
    }
}

impl<D, T> IntoError<D> for (StatusCode, T)
where T: Into<Box<Error + 'static>> {
    fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D> {
        let (status_code, err) = self;
        let err = err.into();
        NickelError::new(res, err.description().to_string(), status_code)
    }
}
