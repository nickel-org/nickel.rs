use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Plugin, Pluggable};
use std::io;
use std::io::{Read, ErrorKind};

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }
impl<'a, 'b> Plugin<Request<'a, 'b>> for JsonBodyParser {
    type Error = io::Error;

    fn eval(req: &mut Request) -> io::Result<String> {
        let mut s = String::new();
        try!(req.origin.read_to_string(&mut s));
        Ok(s)
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> io::Result<T>;
}

impl<'a, 'b> JsonBody for Request<'a, 'b> {
    fn json_as<T: Decodable>(&mut self) -> io::Result<T> {
        self.get::<JsonBodyParser>().and_then(|parsed|
            json::decode::<T>(&*parsed).map_err(|err|
                io::Error::new(ErrorKind::Other,
                               "Failed to parse JSON",
                               Some(format!("{}", err)))
            )
        )
    }
}
