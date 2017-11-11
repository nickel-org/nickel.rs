use hyper::Body;
use hyper::StatusCode::{NotFound, BadRequest};
use request::Request;
use responder::Responder;
use response::ResponseStream;
use middleware::{ErrorHandler, Action, Halt};
use nickel_error::NickelError;
use std::io::Write;

#[derive(Clone, Copy)]
pub struct DefaultErrorHandler;

impl<D> ErrorHandler<D> for DefaultErrorHandler {
    fn handle_error(&self, err: &mut NickelError<D>, _req: &mut Request<D>) -> Action {
        if let Some(ref mut res) = err.stream {
            println!("Default Error: status = {:?}", res.status());
            let msg : &[u8] = match res.status() {
                NotFound => b"Not Found",
                BadRequest => b"Bad Request",
                _ => b"Internal Server Error"
            };

            let body: ResponseStream = Box::new(Body::from(msg));
            res.origin.set_body(body);
        } else {
            error!("Default Error w/o response: {}", err.message);
        }
        Halt(())
    }
}
