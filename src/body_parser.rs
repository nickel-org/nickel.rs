use body_transformer::{BodyError, BodyTransformer};
use futures::Future;
use futures::stream::Stream;
use hyper::{Body, Chunk};
use hyper::error::Error as HyperError;
use hyper::header::ContentType;
use hyper::mime::APPLICATION_WWW_FORM_URLENCODED;
use serialize::{Decodable, json};
use request::Request;
use plugin::{Plugin, Pluggable};
use status::StatusCode;
use std::io::{self, ErrorKind};
use typemap::Key;
use urlencoded::{self, Params};

struct BodyReader;

impl Key for BodyReader {
    type Value = String;
}

impl<'mw, D> Plugin<Request<'mw, Body, D>> for BodyReader {
    type Error = io::Error;

    fn eval(req: &mut Request<'mw, Body, D>) -> Result<String, io::Error> {
        match req.string_future() {
            Ok(f) => f.wait(). // sychronizes the async code with serious performance impact
                map_err(|e| io::Error::new(ErrorKind::Other, format!("Hyper Error: {:?}", e))).
                and_then(|r| r.map_err(|_| io::Error::new(ErrorKind::InvalidData, "Body not Utf8"))),
            Err(BodyError::MissingBody) => Err(io::Error::new(ErrorKind::NotFound, "Body Missing")),
            Err(e) => Err(io::Error::new(ErrorKind::Other, format!("Unexpected Error: {:?}", e))), // shouldn't happen
        }
    }
}

struct FormBodyParser;

impl Key for FormBodyParser {
    type Value = Params;
}

impl<'mw, D> Plugin<Request<'mw, Body, D>> for FormBodyParser {
    type Error = BodyError;

    fn eval(req: &mut Request<Body, D>) -> Result<Params, BodyError> {
        match req.origin.headers().get::<ContentType>() {
            Some(&ContentType(ref t)) => {
                if t.type_() != APPLICATION_WWW_FORM_URLENCODED.type_() || t.subtype() != APPLICATION_WWW_FORM_URLENCODED.subtype() {
                    return Err(BodyError::WrongContentType);
                }
                let body = try!(req.get_ref::<BodyReader>());
                Ok(urlencoded::parse(&*body))
            },
            _ => Err(BodyError::WrongContentType)
        }
    }
}

#[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
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

#[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> FormBody for Request<'mw, B, D> {
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)> {
        self.get_ref::<FormBodyParser>().map_err(|e| (StatusCode::BadRequest, e))
    }
}

#[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error>;
}

#[deprecated(since = "0.11.0", note="Synchronizes async code with performance impact, will be removed in 0.12")]
impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> JsonBody for Request<'mw, B, D> {
    // FIXME: Update the error type.
    // Would be good to capture parsing error rather than a generic io::Error.
    // FIXME: Do the content-type check
    fn json_as<T: Decodable>(&mut self) -> Result<T, io::Error> {
        self.get_ref::<BodyReader>().and_then(|body|
            json::decode::<T>(&*body).map_err(|err|
                io::Error::new(ErrorKind::Other, format!("Parse error: {}", err))
            )
        )
    }
}
