use std::borrow::Cow;
use hyper::status::StatusCode;
use std::io;
use std::error::Error;
use {Response, Request};
use hyper::net::{Fresh, Streaming};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<'a, 'k: 'a, D: 'a> {
    stream: Result<Response<'a, 'k, D, Streaming>, Request<'a, 'k>>,
    pub message: Cow<'static, str>
}

impl<'a, 'k, D> NickelError<'a, 'k, D> {
    /// Creates a new `NickelError` instance.
    ///
    /// You should probably use `Response#error` in favor of this.
    ///
    /// # Examples
    /// ```{rust}
    /// # extern crate nickel;
    ///
    /// # fn main() {
    /// use nickel::{Response, MiddlewareResult, NickelError};
    /// use nickel::status::StatusCode;
    ///
    /// # #[allow(dead_code)]
    /// fn handler<'a, 'k, D>(res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
    ///     Err(NickelError::new(res, "Error Parsing JSON", StatusCode::BadRequest))
    /// }
    /// # }
    /// ```
    pub fn new<T>(mut stream: Response<'a, 'k, D, Fresh>,
                  message: T,
                  status_code: StatusCode) -> NickelError<'a, 'k, D>
            where T: Into<Cow<'static, str>> {
        stream.set(status_code);

        match stream.start() {
            Ok(stream) =>
                NickelError {
                    stream: Ok(stream),
                    message: message.into(),
                },
            Err(e) => e
        }
    }

    pub fn response(&self) -> Option<&Response<'a, 'k, D, Streaming>> {
        self.stream.as_ref().ok()
    }

    pub fn request(&self) -> &Request<'a, 'k> {
        match self.stream {
            Ok(ref res) => &res.request,
            Err(ref req) => req,
        }
    }

    pub fn response_mut(&mut self) -> Option<&mut Response<'a, 'k, D, Streaming>> {
        self.stream.as_mut().ok()
    }

    pub fn request_mut(&mut self) -> &mut Request<'a, 'k> {
        match self.stream {
            Ok(ref mut res) => &mut res.request,
            Err(ref mut req) => req,
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
    pub unsafe fn without_response<T>(message: T, request: Request<'a, 'k>) -> NickelError<'a, 'k, D>
            where T: Into<Cow<'static, str>> {
        NickelError {
            stream: Err(request),
            message: message.into(),
        }
    }

    pub fn end(self) -> Option<io::Result<()>> {
        self.stream.ok().map(|s| s.end())
    }
}

impl<'a, 'k, T, D> From<(Response<'a, 'k, D>, (StatusCode, T))> for NickelError<'a, 'k, D>
        where T: Into<Box<Error + 'static>> {
    fn from((res, (errorcode, err)): (Response<'a, 'k, D>, (StatusCode, T))) -> NickelError<'a, 'k, D> {
        let err = err.into();
        NickelError::new(res, err.description().to_string(), errorcode)
    }
}

impl<'a, 'k, D> From<(Response<'a, 'k, D>, String)> for NickelError<'a, 'k, D> {
    fn from((res, msg): (Response<'a, 'k, D>, String)) -> NickelError<'a, 'k, D> {
        NickelError::new(res, msg, StatusCode::InternalServerError)
    }
}

impl<'a, 'k, D> From<(Response<'a, 'k, D>, StatusCode)> for NickelError<'a, 'k, D> {
    fn from((res, code): (Response<'a, 'k, D>, StatusCode)) -> NickelError<'a, 'k, D> {
        NickelError::new(res, "", code)
    }
}
