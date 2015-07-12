use hyper::status::StatusCode::{NotFound, BadRequest};
use middleware::{ErrorHandler, Action, Halt};
use nickel_error::NickelError;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl<D> ErrorHandler<D> for DefaultErrorHandler {
    fn handle_error(&self, err: &mut NickelError<D>) -> Action {
        // Not sure why the borrow isn't being released for the None branch
        if err.response().is_none() { println!("Error: {}", err.message) }

        if let Some(ref mut res) = err.response_mut() {
            let msg : &[u8] = match res.status() {
                NotFound => b"Not Found",
                BadRequest => b"Bad Request",
                _ => b"Internal Server Error"
            };

            let _ = res.write_all(msg);
        }

        Halt(())
    }
}
