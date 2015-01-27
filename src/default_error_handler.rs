use http::status::{NotFound, BadRequest, InternalServerError};
use request::Request;
use response::Response;
use ResponseFinalizer;
use middleware::{ErrorHandler, MiddlewareResult};
use nickel_error::{NickelError, ErrorWithStatusCode};

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn invoke(&self, err: &NickelError, _req: &mut Request, res: &mut Response) -> MiddlewareResult {
        let r = match err.kind {
            ErrorWithStatusCode(NotFound) => (NotFound, "Not Found"),
            ErrorWithStatusCode(BadRequest) => (BadRequest, "Bad Request"),
            _ => (InternalServerError, "Internal Server Error")
        };

        r.respond(res)
    }
}
