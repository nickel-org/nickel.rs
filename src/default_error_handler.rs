use http::status::{ NotFound, BadRequest, InternalServerError };
use request::Request;
use response::Response;
use middleware::{Halt, ErrorHandler, MiddlewareResult};
use nickel_error::{ NickelError, ErrorWithStatusCode };

#[deriving(Clone)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn invoke(&self, err: &NickelError, _req: &mut Request, res: &mut Response) -> MiddlewareResult {
        match err.kind {
            ErrorWithStatusCode(NotFound) => {
                res.origin.status = NotFound;
                res.send("Not Found");
                Ok(Halt)
            },
            ErrorWithStatusCode(BadRequest) => {
                res.origin.status = BadRequest;
                res.send("Bad Request");
                Ok(Halt)
            }
            _ => {
                res.origin.status = InternalServerError;
                res.send("Internal Server Error");
                Ok(Halt)
            }
        }
    }
}
