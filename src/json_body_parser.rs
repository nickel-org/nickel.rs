use std::str;
use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Phantom, Plugin, Pluggable};

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }
impl<'a, 'b> Plugin<Request<'a, 'b>> for JsonBodyParser {
    fn eval(req: &mut Request, _: Phantom<JsonBodyParser>) -> Option<String> {
        if !req.origin.body.is_empty() {
            str::from_utf8(&*req.origin.body).ok().map(|s| s.to_string())
        } else {
            None
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
            json::decode::<T>(&parsed[]).ok()
        })
    }
}
