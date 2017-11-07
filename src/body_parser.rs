use futures::Future;
use futures::stream::Stream;
use hyper::Chunk;
use hyper::error::Error as HyperError;
use hyper::header::ContentType;
use hyper::mime::APPLICATION_WWW_FORM_URLENCODED;
use serialize::{Decodable, json};
use request::Request;
use plugin::{Plugin, Pluggable};
use status::StatusCode;
use std::error::Error as StdError;
use std::fmt;
use std::io::{self, ErrorKind, Read};
use std::str::{from_utf8, Utf8Error};
use typemap::Key;
use urlencoded::{self, Params};

type BodyFuture = Box<Future<Item = String, Error = Utf8Error>>;

struct BodyReader;

impl Key for BodyReader {
    type Value = BodyFuture;
}

impl<'mw, B: Stream<Item = Chunk, Error = HyperError>, D> Plugin<Request<'mw, B, D>> for BodyReader {
    type Error = HyperError;

    // This doesn't quite work. The Key trait requires that BodyFuture
    // have a 'static lifetime, but we can only garantee 'mw. In
    // practice this shouldn't be an issue, but I don't see a way to
    // convince the compiler of it, at least in safe rust.
    //
    // Two thoughts:
    //
    //  1. Can we completely remove the 'mw lifetime at this point? would that help?
    //  2. Give up on the plugin approach, and make StringBody trait
    //     on Request that returns a BodyFuture, similar to how
    //     JsonBody is managed.
    //
    // Current plan:
    //
    //  1. Implement BodyFuture, ParamFuture, and JsonFuture to
    //     provide futures returning those values. This will be the
    //     new interface.
    //  2. Implement the old plugins using the new *Futures and
    //     wait. This will be very slow and possibly problematic, so
    //     they will be immediately deprecated.
    fn eval(req: &mut Request<'mw, B, D>) -> Result<Box<Future<Item = String, Error = Utf8Error> + 'static>, HyperError> {
        req.origin.body_ref().ok_or(HyperError::Incomplete).
            map(|b| {
                b.concat2().and_then(|body| {
                    from_utf8(&body).map(|s| s.to_string())
                })
            }).map(|bs| Box::new(bs))
    }
}

struct FormBodyParser;

impl Key for FormBodyParser {
    type Value = Params;
}

impl<'mw, B: Stream<Item = Chunk, Error = HyperError>, D> Plugin<Request<'mw, B, D>> for FormBodyParser {
    type Error = BodyError;

    fn eval(req: &mut Request<B, D>) -> Result<Params, BodyError> {
        match req.origin.headers().get::<ContentType>() {
            Some(&ContentType(APPLICATION_WWW_FORM_URLENCODED)) => {
                let body = try!(req.get_ref::<BodyReader>());
                Ok(urlencoded::parse(&*body))
            },
            _ => Err(BodyError::WrongContentType)
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

impl<'mw, B: Stream<Item = Chunk, Error = HyperError>, D> FormBody for Request<'mw, B, D> {
    fn form_body(&mut self) -> Result<&Params, (StatusCode, BodyError)> {
        self.get_ref::<FormBodyParser>().map_err(|e| (StatusCode::BadRequest, e))
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Result<T, HyperError>;
}

impl<'mw, B: Stream<Item = Chunk, Error = HyperError>, D> JsonBody for Request<'mw, B, D> {
    // FIXME: Update the error type.
    // Would be good to capture parsing error rather than a generic io::Error.
    // FIXME: Do the content-type check
    fn json_as<T: Decodable>(&mut self) -> Result<T, HyperError> {
        self.get_ref::<BodyReader>().and_then(|body|
            json::decode::<T>(&*body).map_err(|err|
                HyperError::Io(io::Error::new(ErrorKind::Other, format!("Parse error: {}", err)))
            )
        )
    }
}
