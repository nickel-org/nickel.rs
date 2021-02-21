use hyper::StatusCode;
use crate::request::Request;
use crate::middleware::{ErrorHandler, Action, Halt};
use crate::nickel_error::NickelError;

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl<D: Send + 'static + Sync> ErrorHandler<D> for DefaultErrorHandler {
    fn handle_error(&self, err: &mut NickelError<D>, _req: &mut Request<D>) -> Action {
        if let Some(ref mut res) = err.stream {
            let msg : &[u8] = match res.status() {
                StatusCode::NOT_FOUND => b"Not Found",
                StatusCode::BAD_REQUEST => b"Bad Request",
                _ => b"Internal Server Error"
            };

            let _ = res.set_body(msg);
        } else {
            println!("Error: {}", err.message);
        }

        Halt(())
    }
}
