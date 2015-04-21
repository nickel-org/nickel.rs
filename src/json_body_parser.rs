use serialize::{Decodable, json};
use request::Request;
use typemap::Key;
use plugin::{Plugin, Pluggable};
use std::io;
use std::io::{Read, ErrorKind};
use std::rc::Rc;

// Plugin boilerplate
struct JsonBodyParser;
impl Key for JsonBodyParser { type Value = String; }
impl<'a, 'b, 'k> Plugin<Request<'a, 'b, 'k>> for JsonBodyParser {
    // FIXME: Plugin requires Error to be `Clone`, but we can probably
    // do something so we don't need to have an `Rc`
    type Error = Rc<io::Error>;

    fn eval(req: &mut Request) -> Result<String, Rc<io::Error>> {
        let mut s = String::new();
        try!(req.origin.read_to_string(&mut s)
                       .map_err(Rc::new));
        Ok(s)
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable>(&mut self) -> Result<T, Rc<io::Error>>;
}

impl<'a, 'b, 'k> JsonBody for Request<'a, 'b, 'k> {
    fn json_as<T: Decodable>(&mut self) -> Result<T, Rc<io::Error>> {
        self.get::<JsonBodyParser>().and_then(|parsed|
            json::decode::<T>(&*parsed).map_err(|_err|
                Rc::new(io::Error::new(ErrorKind::Other,
                                   format!("Failed to parse JSON: {}", _err)))
                               
            )
        )
    }
}
