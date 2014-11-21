use std::str;
use serialize::json;
use serialize::Decodable;
use serialize::json::{ Json, Decoder, DecoderError};
use http::status::BadRequest;
use request;
use request::Request;
use response::Response;
use middleware::{Continue, Middleware, MiddlewareResult};
use nickel_error::{ NickelError, ErrorWithStatusCode };
use typemap::Assoc;
use plugin::{Phantom, PluginFor, GetCached};

// Plugin boilerplate
struct JsonBodyParser;
impl Assoc<String> for JsonBodyParser {}
impl<'a, 'b> PluginFor<Request<'a, 'b>, String> for JsonBodyParser {
    fn eval(req: &mut Request, _: Phantom<JsonBodyParser>) -> Option<String> {
        if !req.origin.body.is_empty() {
            str::from_utf8(req.origin.body.as_slice()).map(|s| s.to_string())
        } else {
            None
        }
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable<Decoder,DecoderError>>(&mut self) -> Option<T>;
}

impl<'a, 'b> JsonBody for request::Request<'a, 'b> {
    fn json_as<T: Decodable<Decoder, DecoderError>>(&mut self) -> Option<T> {
        // FIXME:
        // I think it would be smarter to not return Option<T> but rather
        // DecodeResult<T> to not swallow valuable debugging information.
        // I couldn't figure out how to properly do that
        self.get::<JsonBodyParser>().and_then(|parsed| {
            ::serialize::json::decode::<T>(parsed.as_slice()).ok()
        })
    }
}
