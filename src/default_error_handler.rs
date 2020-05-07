use hyper::StatusCode;
use crate::request::Request;
use crate::middleware::{ErrorHandler, Action, Halt};
use crate::nickel_error::NickelError;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl<D> ErrorHandler<D> for DefaultErrorHandler {
    fn handle_error(&self, err: &mut NickelError<'_, D>, _req: &mut Request<'_, '_, D>) -> Action {
        if let Some(ref mut res) = err.stream {
            let msg : &[u8] = match res.status() {
                StatusCode::NOT_FOUND => b"Not Found",
                StatusCode::BAD_REQUEST => b"Bad Request",
                _ => b"Internal Server Error"
            };

            let _ = res.write_all(msg);
        } else {
            println!("Error: {}", err.message);
        }

        Halt(())
    }
}
