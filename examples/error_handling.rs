#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter};
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

impl<'a> From<&'a AppError> for StatusCode {
  fn from(err: &AppError) -> StatusCode {
    match *err {
      AppError::Custom(_) => StatusCode::BadRequest,
      AppError::Io(_)     => StatusCode::InternalServerError
    }
  }
}

type AppResult<T> = Result<T,AppError>;

fn will_fail() -> AppResult<String> {
  Err(AppError::Custom("uups".to_string()))
}

fn will_work() -> AppResult<String> {
  Ok("foo".to_string())
}

fn foo_handler<'a, D>(_: &mut Request<D>, mut res: Response<'a,D>) -> MiddlewareResult<'a, D> {

  let x: AppResult<String> = (||Ok({
    try!(will_fail());
    try!(will_work())
  }))();

  match x {
    Ok(s)    => res.send(s),
    Err(ref err) => {
      res.set(StatusCode::from(err));
      res.send(err.to_string())
    }
  }
}

fn main() {
  let mut server = Nickel::new();
  server.get("foo/", foo_handler);
  server.listen("127.0.0.1:6767");
}
