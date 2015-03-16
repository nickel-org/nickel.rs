use std::borrow::{IntoCow, Cow};
use hyper::status::StatusCode;
use std::io;
use response::Response;
use hyper::net::{Fresh, Streaming};

/// NickelError is the basic error type for HTTP errors as well as user defined errors.
/// One can pattern match against the `kind` property to handle the different cases.
pub struct NickelError<'a> {
    pub stream: Option<Response<'a, Streaming>>,
    pub message: Cow<'static, str>
}

impl<'a> NickelError<'a> {
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
    /// fn handler<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
    ///     Err(NickelError::new(res,
    ///                          "Error Parsing JSON",
    ///                          StatusCode::BadRequest))
    /// }
    /// # }
    /// ```
    pub fn new<T>(mut stream: Response<'a, Fresh>,
                  message: T,
                  status_code: StatusCode) -> NickelError<'a>
            where T: IntoCow<'static, str> {
        stream.set_status(status_code);

        match stream.start() {
            Ok(stream) => {
                NickelError {
                    stream: Some(stream),
                    message: message.into_cow(),
                }
            },
            Err(e) => e
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
    pub unsafe fn without_response<T>(message: T) -> NickelError<'a>
            where T: IntoCow<'static, str> {
        let message = message.into_cow();
        println!("Error: {}", message);
        NickelError {
            stream: None,
            message: message,
        }
    }

    pub fn end(self) -> Option<io::Result<()>> {
        self.stream.map(|s| s.end())
    }
}
