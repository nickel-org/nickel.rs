#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter, NickelError};
use nickel::{Action, IntoError};
use nickel::status::StatusCode;
use rustc_serialize::json::{self, Json};
use std::{error,io, fmt};
use std::io::Write;

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
  Ok("Everything went to plan!")
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

  server.handle_error(|err: &mut NickelError<_>, req: &mut Request<_>| {
    // Print the internal error message and path to the console
    println!("[{}] ERROR: {}",
            req.path_without_query().unwrap(),
            err.message);

    // If we still have a stream available then render a customised response
    // for some StatusCodes.
    if let Some(ref mut res) = err.stream {
      match res.status() {
        StatusCode::ImATeapot => {
          // Discard the result as if it fails there's not much we can do.
          let _ = res.write_all(b"<h2>I'm a Teapot!</h2>");
        }
        StatusCode::NotFound => {
          let _ = res.write_all(b"<h1>404 - Not Found</h1>");
        }
        // Let the next ErrorHandler do something.
        _ => return Action::Continue(())
      }
    }

    // Send nothing else, just let the client interpret the StatusCode.
    Action::Halt(())
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
