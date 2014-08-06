use serialize::json;
use serialize::Decodable;
use serialize::json::{ Json, Decoder, DecoderError};
use request;
use request::Request;
use response::Response;
use middleware::{Action, Continue, Middleware};
use nickel_error::{ NickelError, Other };

#[deriving(Clone)]
pub struct JsonBodyParser;

impl Middleware for JsonBodyParser {
    fn invoke (&self, req: &mut Request, _res: &mut Response) -> Result<Action, NickelError> {

        if !req.origin.body.is_empty() {
            match json::from_str(req.origin.body.as_slice()) {
                Ok(parsed) => {
                    req.map.insert(parsed);
                    return Ok(Continue);
                },
                Err(_) => {
                    // TODO: Should this really be an error then?
                    // I actually rather think *no* because we would handle this error
                    // directly in this middleware and then return Ok(Halt) no?
                    // Isn't it that we should always handle errors that are native to the
                    // middleware directly *inside* the middleware and only pass on errors
                    // that are not native to the middleware in question. E.g a static file handler
                    // middleware should handle 404s and return ok(Halt) but should pass on other errors?
                    return Err(NickelError::new("Error parsing JSON", Other));
                }
            }
        }
        Ok(Continue)
    }
}

impl<'a> request::Request<'a> {
    pub fn json_as<T: Decodable<Decoder,DecoderError>>(& self) -> Option<T>{

        // FIXME:
        // I think it would be smarter to not return Option<T> but rather
        // DecodeResult<T> to not swallow valuable debugging information. 
        // I couldn't figure out how to properly do that

        self.map.find::<Json>()
                .and_then(| parsed | { 
                    match ::serialize::json::decode::<T>(parsed.to_string().as_slice()) {
                        Ok(e) => Some(e),
                        _ => None
                    }
                })
    }
}
