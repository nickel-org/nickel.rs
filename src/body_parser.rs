use serialize::{Decodable, json};
use request::Request;
use plugin::{Plugin, Pluggable};
use hyper::StatusCode;
use std::error::Error as StdError;
use std::fmt;
use std::io::{self, ErrorKind};
use typemap::Key;
use urlencoded::Params;
use urlencoded;
use hyper::header::ContentType;
use hyper::mime;

struct BodyReader;

impl Key for BodyReader {
    type Value = String;
}

impl<'mw, D> Plugin<Request<'mw, D>> for BodyReader {
    type Error = io::Error;

    fn eval(req: &mut Request<D>) -> Result<String, io::Error> {
    
        match req.body() {
            Some(body) => {
                let body_as_string = String::from_utf8(body).unwrap();
                Ok(body_as_string)
            },
            None => Ok("".to_owned())
        }
    }
}

struct FormBodyParser;

impl Key for FormBodyParser {
    type Value = Params;
}

impl<'mw, D> Plugin<Request<'mw, D>> for FormBodyParser {
    type Error = BodyError;

    fn eval(req: &mut Request<D>) -> Result<Params, BodyError> {
        match req.origin.headers.get::<ContentType>() {
            Some(&ContentType(ref mime1)) => {
                if mime1.type_() == mime::APPLICATION && mime1.subtype() == mime::WWW_FORM_URLENCODED {
                    let body = try!(req.get_ref::<BodyReader>());
                    Ok(urlencoded::parse(&*body))
                } else {
                    Err(BodyError::WrongContentType)
                }
            }
            _ => Err(BodyError::WrongContentType),
        }
    }
}

pub trait FormBody {
    /// Extracts URL encoded data from the request body.
    /// # Examples
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// use nickel::{Nickel, HttpRouter, FormBody};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.post("/a", middleware! { |req, res|
    ///         let form_body = try_with!(res, req.form_body());
    ///         return res.send(format!("Post: {:?}", form_body))
    ///     });
    /// }
    /// ```
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)>;
}

impl<'mw, D> FormBody for Request<'mw, D> {
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)> {
        self.get_ref::<FormBodyParser>().map_err(|e| (StatusCode::BadRequest, e))
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error>;
}

impl<'mw, D> JsonBody for Request<'mw, D> {
    // FIXME: Update the error type.
    // Would be good to capture parsing error rather than a generic io::Error.
    // FIXME: Do the content-type check
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error> {
        self.get_ref::<BodyReader>().and_then(|body| {
            println!("{:?}", body);
            json::decode::<T>(&*body)
                .map_err(|err| io::Error::new(ErrorKind::Other, format!("Parse error: {}", err)))
        })
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
    fn description(&self) -> &str {
        match *self {
            BodyError::Io(ref err) => err.description(),
            BodyError::WrongContentType => "Wrong content type",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            BodyError::Io(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description())
    }
}
