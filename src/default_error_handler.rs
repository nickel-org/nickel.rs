use hyper::status::StatusCode::{NotFound, BadRequest};
use request::Request;
use middleware::{ErrorHandler, Action, Halt};
use nickel_error::{NickelError, ErrorWithStatusCode};
use std::old_io::Writer;

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, err: &mut NickelError, _req: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            let msg = match err.kind {
                ErrorWithStatusCode(NotFound) => b"Not Found",
                ErrorWithStatusCode(BadRequest) => b"Bad Request",
                _ => b"Internal Server Error"
            };

            let _ = res.write_all(msg);
        }

        Halt(())
    }
}
