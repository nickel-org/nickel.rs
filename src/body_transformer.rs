use futures::Future;
use futures::stream::Stream;
use hyper::Chunk;
use hyper::error::Error as HyperError;
use hyper::header::ContentType;
use hyper::mime::APPLICATION_WWW_FORM_URLENCODED;
use serialize::Decodable;
use serialize::json::{DecodeResult, DecoderError, ParserError, ErrorCode, decode};
use request::Request;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str::{from_utf8, Utf8Error};
use urlencoded::{parse_bytes, Params};

type StringFuture<'mw> = Box<Future<Item=Result<String, Utf8Error>, Error=HyperError> + 'mw>;
type ParamsFuture<'mw> = Box<Future<Item=Params, Error=HyperError> + 'mw>;
type JsonFuture<'mw, T: Decodable> = Box<Future<Item=DecodeResult<T>, Error=HyperError> + 'mw>;

pub trait BodyTransformer {
    // Could this be StrFuture wrapping an &str instead of a String,
    // and avoid an allocation?
    fn string_future(&self) -> Result<StringFuture, BodyError>;
    fn params_future(&self) -> Result<ParamsFuture, BodyError>;
    fn json_future<T: Decodable>(&self) -> Result<JsonFuture<T>, BodyError>;
}

impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> BodyTransformer for Request<'mw, B, D> {
    fn string_future(&self) -> Result<StringFuture, BodyError> {
        let future: Box<Future<Item=_, Error=_>> = Box::new(match self.origin.body_ref()
        {
            Some(b) => b.concat2().map(|body| {
                from_utf8(&body).map(|s| s.to_string())
            }),
            None => { return Err(BodyError::MissingBody); }
        });
        Ok(future)
    }

    fn params_future(&self) -> Result<ParamsFuture, BodyError> {
        match self.origin.headers().get::<ContentType>() {
            Some(&ContentType(APPLICATION_WWW_FORM_URLENCODED)) => {
                let future: Box<Future<Item=_, Error=_>> = Box::new( match self.origin.body_ref() {
                    Some(b) => b.concat2().map(|body| {
                        parse_bytes(&body)
                    }),
                    None => { return Err(BodyError::MissingBody); }
                });
                Ok(future)
            },
            _ => Err(BodyError::WrongContentType)
        }
    }

    fn json_future<T: Decodable>(&self) -> Result<JsonFuture<T>, BodyError> {
        // Todo: Add a content type check here. What will that break?
        let future: Box<Future<Item=_, Error=_>> = Box::new(match self.origin.body_ref()
        {
            Some(b) => b.concat2().map(|body| {
                from_utf8(&body).
                    map_err(|_| DecoderError::ParseError(ParserError::SyntaxError(ErrorCode::NotUtf8, 0, 0))).
                    and_then(|s| decode::<T>(s))
            }),
            None => { return Err(BodyError::MissingBody); }
        });
        Ok(future)
    }
}


#[derive(Debug)]
pub enum BodyError {
    Io(io::Error),
    Hyper(HyperError),
    MissingBody,
    WrongContentType,
}

impl From<io::Error> for BodyError {
    fn from(err: io::Error) -> BodyError {
        BodyError::Io(err)
    }
}

impl From<HyperError> for BodyError {
    fn from(err: HyperError) -> BodyError {
        BodyError::Hyper(err)
    }
}

impl StdError for BodyError {
    fn description(&self) -> &str {
        match *self {
            BodyError::Io(ref err) => err.description(),
            BodyError::Hyper(ref err) => err.description(),
            BodyError::WrongContentType => "Wrong content type"
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            BodyError::Io(ref err) => Some(err),
            BodyError::Hyper(ref err) => Some(err),
            _ => None
        }
    }
}

impl fmt::Display for BodyError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description())
    }
}
