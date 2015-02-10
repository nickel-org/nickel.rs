use std::str;
use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Plugin, Pluggable};

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }
impl<'a, 'b> Plugin<Request<'a, 'b>> for JsonBodyParser {
    type Error = &'static str;

    fn eval(req: &mut Request) -> Result<String, &'static str> {
        if !req.origin.body.is_empty() {
            str::from_utf8(&*req.origin.body)
                .map(|s| s.to_string())
                .map_err(|_| "Request body is not utf-8.")
        }
        else {
            Err("Request body is empty.")
        }
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
            json::decode::<T>(&parsed[]).map_err(|_| "Couldn't parse JSON.")
        }).ok()
    }
}
