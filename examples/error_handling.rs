#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter, NickelError};
use nickel::IntoError;
use nickel::status::StatusCode;
use std::{error,io, fmt};

#[derive(Debug)]
pub enum AppError {
  Io(io::Error),
  Custom(String)
}

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

type AppResult<T> = Result<T,AppError>;

fn will_fail() -> AppResult<String> {
  Err(AppError::Custom("uups".to_string()))
}

fn will_work() -> AppResult<&'static str> {
  Ok("foo")
}

fn success_handler<'a, D>(_: &mut Request<D>, res: Response<'a,D>) -> MiddlewareResult<'a, D> {
  let msg = try_with!(res, will_work());

  res.send(msg)
}

fn failure_handler<'a, D>(_: &mut Request<D>, res: Response<'a,D>) -> MiddlewareResult<'a, D> {
  let msg = try_with!(res, will_fail());

  res.send(msg)
}

fn main() {
  let mut server = Nickel::new();
  server.get("/failure", failure_handler);
  server.get("/success", success_handler);

  server.listen("127.0.0.1:6767");
}
