#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter, NickelError};
use nickel::IntoError;
use nickel::status::StatusCode;
use rustc_serialize::json::{self, Json};
use std::{error,io, fmt};

/// The crate-local server data type.
struct MyData;

/// An example of a crate-local error type.
#[derive(Debug)]
pub enum AppError {
  Io(io::Error),
  Custom(String)
}

/// Type alias for convenient use of the crate-local error type.
type AppResult<T> = Result<T, AppError>;

/// If the type you're implementing the trait for is crate-local, then you can
/// make your `IntoError` implementation generic over the ServerData type if you
/// want to. This can be good if you want to create an error type in a library
/// which can be used by servers with any data type.
impl<D> IntoError<D> for AppError {
  fn into<'a>(self, res: Response<'a, D>) -> NickelError<'a, D> {
    let internal_msg = format!("{}", self);

    let status_code = match self {
      AppError::Custom(_) => StatusCode::BadRequest,
      AppError::Io(_)     => StatusCode::InternalServerError
    };

    NickelError::new(res, internal_msg, status_code)
  }
}

/// By using a local type as the ServerData type, you can implement `IntoError`
/// for foreign types. This example uses the `json::ParserError` type which is
/// used whenever `Json::from_str` fails.
impl IntoError<MyData> for json::ParserError {
  fn into<'a>(self, res: Response<'a, MyData>) -> NickelError<'a, MyData> {
    NickelError::new(res, "Failed to create json", StatusCode::ImATeapot)
  }
}

fn will_fail() -> AppResult<String> {
  Err(AppError::Custom("Something went wrong!".to_string()))
}

fn will_work() -> AppResult<&'static str> {
  Ok("foo")
}

fn success_handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
  let msg = try_with!(res, will_work());

  res.send(msg)
}

fn failure_handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
  let msg = try_with!(res, will_fail());

  res.send(msg)
}

/// Note that this handler explicitly defined its data type so that the `IntoError` impl
/// is able to be used.
fn json_handler<'a>(_: &mut Request<MyData>, res: Response<'a, MyData>)
                    -> MiddlewareResult<'a, MyData> {
  let json = try_with!(res, Json::from_str("Not json"));

  res.send(json)
}

fn main() {
  let mut server = Nickel::with_data(MyData);
  server.get("/failure", failure_handler);
  server.get("/success", success_handler);
  server.get("/json_nomacro", json_handler);
  server.get("/json", middleware! { |_, res|
    let json = try_with!(res, Json::from_str("Not json"));

    return res.send(json)
  });

  server.listen("127.0.0.1:6767").unwrap();
}


//
// Other trait implementations for AppError
//

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      AppError::Custom(ref msg) => write!(f, "Custom error: {}", msg),
      AppError::Io(ref err)     => write!(f, "IO error: {}", err)
    }
  }
}

impl error::Error for AppError {
  fn description(&self) -> &str {
    match *self {
      AppError::Custom(ref msg) => msg,
      AppError::Io(ref err)     => err.description()
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    match *self {
      AppError::Custom(_)   => None,
      AppError::Io(ref err) => Some(err)
    }
  }
}
