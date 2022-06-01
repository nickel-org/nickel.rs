// WARNING: this module is no longer used, and is only being kept around for
// documentation as part of migration to async.

use hyper::header::ContentType;
use hyper::mime::{Mime, SubLevel, TopLevel};
use serde::Deserialize;
use serde_json;
use crate::request::Request;
use plugin::{Plugin, Pluggable};
use crate::status::StatusCode;
use std::error::Error as StdError;
use std::fmt;
use std::io::{self, ErrorKind, Read};
use typemap::Key;
use crate::urlencoded::{self, Params};

struct BodyReader;

impl Key for BodyReader {
    type Value = String;
}

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for BodyReader {
    type Error = io::Error;

    fn eval(req: &mut Request<'_, '_, D>) -> Result<String, io::Error> {
        let mut buf = String::new();
        req.origin.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

struct FormBodyParser;

impl Key for FormBodyParser {
    type Value = Params;
}

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for FormBodyParser {
    type Error = BodyError;

    fn eval(req: &mut Request<'_, '_, D>) -> Result<Params, BodyError> {
        match req.origin.headers.get::<ContentType>() {
            Some(&ContentType(Mime(
                TopLevel::Application,
                SubLevel::WwwFormUrlEncoded,
                _
            ))) => {
                let body = req.get_ref::<BodyReader>()?;
                Ok(urlencoded::parse(&*body))
            },
            _ => Err(BodyError::WrongContentType)
        }
    }
}

pub trait FormBody {
    /// Extracts URL encoded data from the request body.
    ///
    /// # Examples
    ///
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// use nickel::{Nickel, HttpRouter, FormBody};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.post("/a", middleware! { |req, res|
    ///         let form_body = try_with!(res, req.form_body());
    ///         format!("Post: {:?}", form_body)
    ///     });
    /// }
    /// ```
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)>;
}

impl<'mw, 'conn, D> FormBody for Request<'mw, 'conn, D> {
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)> {
        self.get_ref::<FormBodyParser>().map_err(|e| (StatusCode::BadRequest, e))
    }
}

pub trait JsonBody {
    fn json_as<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T, io::Error>;
}

impl<'mw, 'conn, D> JsonBody for Request<'mw, 'conn, D> {
    // FIXME: Update the error type.
    // Would be good to capture parsing error rather than a generic io::Error.
    // FIXME: Do the content-type check
    fn json_as<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T, io::Error> {
        self.get_ref::<BodyReader>().and_then(|body|
            serde_json::from_str::<T>(&*body).map_err(|err|
                io::Error::new(ErrorKind::Other, format!("Parse error: {}", err))
            )
        )
    }
}

#[derive(Debug)]
pub enum BodyError {
    Io(io::Error),
    WrongContentType,
}

impl From<io::Error> for BodyError {
    fn from(err: io::Error) -> BodyError {
        BodyError::Io(err)
    }
}

impl StdError for BodyError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            BodyError::Io(ref err) => Some(err),
            BodyError::WrongContentType => None
        }
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            BodyError::Io(ref err) => { write!(out, "{}", err.to_string()) },
            BodyError::WrongContentType => { write!(out, "Wrong content type") },
        }
    }
}

