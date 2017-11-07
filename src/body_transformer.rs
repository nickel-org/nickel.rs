use futures::{Future, IntoFuture};
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

type StringFuture<'mw> = Box<Future<Item=Result<String, Utf8Error>, Error=HyperError> + 'mw>;
// type ParamsFuture<'mw> = Box<Future<Item=Result<Params, Utf8Error>, Error=HyperError> + 'mw>;
// type JsonFuture<'mw> = Box<Future<Item=Result<String, Utf8Error>, Error=HyperError> + 'mw>;

pub trait BodyTransformer {
    fn get_string_future(&self) -> Result<StringFuture, HyperError>;
}

impl<'mw, B: Stream<Item=Chunk, Error=HyperError>, D> BodyTransformer for Request<'mw, B, D> {
    fn get_string_future(&self) -> Result<StringFuture, HyperError> {
        let future: Box<Future<Item=_, Error=_>> = Box::new(match self.origin.body_ref()
        {
            Some(b) => b.concat2().map(|body| {
                from_utf8(&body).map(|s| s.to_string())
            }),
            None => { return Err(HyperError::Incomplete); }
        });
        Ok(future)
    }
}
