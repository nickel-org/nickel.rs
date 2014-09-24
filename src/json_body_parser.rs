use serialize::json;
use serialize::Decodable;
use serialize::json::{ Json, Decoder, DecoderError};
use http::status::BadRequest;
use request;
use request::Request;
use response::Response;
use middleware::{Action, Continue, Middleware};
use nickel_error::{ NickelError, ErrorWithStatusCode };

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
                    return Err(NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest)));
                }
            }
        }
        Ok(Continue)
    }
}

pub trait JsonBody {
    fn json_as<T: Decodable<Decoder,DecoderError>>(& self) -> Option<T>;
}

impl<'a, 'b> JsonBody for request::Request<'a, 'b> {
    fn json_as<T: Decodable<Decoder,DecoderError>>(& self) -> Option<T>{

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
