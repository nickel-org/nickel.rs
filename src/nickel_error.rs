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
/// The main reason to have this trait rather than relying on `Into`
/// is that we need to consume the `Response` itself. If we have generic
/// impls with `(Response, T)` then there's coherence problems for users
/// who wish to implement this for foreign items.
///
/// # Basic usage
/// ```rust
/// #[macro_use]
/// extern crate nickel;
/// extern crate rustc_serialize;
/// use nickel::{Request, Response, MiddlewareResult, JsonBody};
/// use nickel::status::StatusCode;
///
/// #[derive(RustcDecodable)]
/// struct Person {
///     name: String,
/// }
///
/// // An example handler which uses the built-in implementations of `IntoError`
/// // via the `try_with!` macro.
/// # #[allow(unused_assignments)]
/// fn handler<'mw, D>(req: &mut Request<D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
///     let mut person: Person;
///
///     // A String which will be passed to your error handlers.
///     // The StatusCode for the response will be InternalServerError.
///     person = try_with!(res, req.json_as().map_err(|e| e.to_string()));
///
///     // A StatusCode which will be used for the response status.
///     // The error message passed to error handlers will be empty.
///     person = try_with!(res, req.json_as().map_err(|_| StatusCode::BadRequest));
///
///     // A Tuple of (StatusCode, T) where T: Into<Box<std::error::Error>>.
///     // The Error implementation will be used for the message sent to error handlers.
///     person = try_with!(res, req.json_as().map_err(|e| (StatusCode::BadRequest, e)));
///
///     res.send(format!("Hello {}!", person.name))
/// }
///
/// # fn main() {
/// #     use nickel::{HttpRouter, Nickel};
/// #     let mut app = Nickel::new();
/// #     app.get("*", handler);
/// # }
/// ```
///
/// # Custom implementations
///
/// ## Implementing for foreign error types
///
/// With this trait, if a user uses their own `ServerData` type then they can
/// implement this trait for foreign types, e.g. the `json::ParserError` type.
///
/// ```rust
/// extern crate nickel;
/// extern crate rustc_serialize;
///
/// use nickel::{NickelError, Response, IntoError};
/// use nickel::status::StatusCode;
/// use rustc_serialize::json;
///
/// /// The crate-local server data type.
/// struct ServerData;
///
/// // Note the explicit use of `ServerData` instead of a generic parameter.
/// impl IntoError<ServerData> for json::ParserError {
///     fn into<'a>(self, res: Response<'a, ServerData>) -> NickelError<'a, ServerData> {
///         NickelError::new(res, "Failed to create json", StatusCode::ImATeapot)
///     }
/// }
/// # fn main() {}
/// ```
///
/// ## Implementing for local error types
///
/// With local error types, you can implement this generically over the server data
/// type if you want. This is useful for library code where you want the users to be
/// able to choose their own server data types.
///
/// ```rust
/// use std::io;
/// use nickel::{NickelError, Response, IntoError};
/// use nickel::status::StatusCode;
///
/// /// A crate-local error type.
/// #[derive(Debug)]
/// pub enum AppError {
///     Io(io::Error),
///     Custom(String)
/// }
///
/// impl<D> IntoError<D> for AppError {
///     fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D> {
///         let internal_msg = format!("{:?}", self);
///
///         let status_code = match self {
///             AppError::Custom(_) => StatusCode::BadRequest,
///             AppError::Io(_)     => StatusCode::InternalServerError
///         };
///
///         NickelError::new(res, internal_msg, status_code)
///     }
/// }
/// # fn main() {}
/// ```
///
/// See the `error_handling` example for a more complete example.
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
