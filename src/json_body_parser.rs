use std::str;
use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Plugin, Pluggable};
use std::old_io::{IoError, IoResult, OtherIoError};

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }
impl<'a, 'b> Plugin<Request<'a, 'b>> for JsonBodyParser {
    type Error = IoError;

    fn eval(req: &mut Request) -> IoResult<String> {
        req.origin.read_to_string()
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Option<T>;
}

impl<'a, 'b> JsonBody for Request<'a, 'b> {
    fn json_as<T: Decodable>(&mut self) -> Option<T> {
        // FIXME:
        // I think it would be smarter to not return Option<T> but rather
        // DecodeResult<T> to not swallow valuable debugging information.
        // I couldn't figure out how to properly do that
        self.get::<JsonBodyParser>().and_then(|parsed| {
            json::decode::<T>(&*parsed).map_err(|err|
                IoError {
                    kind: OtherIoError,
                    desc: "Failed to parse JSON",
                    detail: Some(format!("{}", err))
                })
        }).ok()
    }
}
