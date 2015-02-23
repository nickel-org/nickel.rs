use request::Request;
use response::Response;
use nickel_error::NickelError;
use middleware_handler::ResponseFinalizer;
use hyper::net;

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<'a, 'b> = Result<Action<Response<'a, 'b, net::Fresh>,
                                                  Response<'a, 'b, net::Streaming>>,
                                           NickelError<'a, 'b>>;

pub enum Action<T=(), U=()> {
    Continue(T),
    Halt(U)
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Send + 'static + Sync {
    fn invoke<'a, 'b>(&'a self, _req: &mut Request<'b, 'a>, res: Response<'a, 'a, net::Fresh>) -> MiddlewareResult<'a, 'a> {
        Ok(Continue(res))
    }
}

pub trait ErrorHandler: Send + 'static + Sync {
    fn handle_error(&self, &mut NickelError, &mut Request) -> Action;
}

impl ErrorHandler for fn(&mut NickelError, &mut Request) -> Action {
    fn handle_error(&self, err: &mut NickelError, req: &mut Request) -> Action {
        (*self)(err, req)
    }
}

pub struct MiddlewareStack {
    handlers: Vec<Box<Middleware + Send + Sync>>,
    error_handlers: Vec<Box<ErrorHandler + Send + Sync>>
}

impl MiddlewareStack {
    pub fn add_middleware<T: Middleware> (&mut self, handler: T) {
        self.handlers.push(Box::new(handler));
    }

    pub fn add_error_handler<T: ErrorHandler> (&mut self, handler: T) {
        self.error_handlers.push(Box::new(handler));
    }

    pub fn invoke<'a>(&'a self, mut req: Request<'a, 'a>, mut res: Response<'a, 'a>) {
        for handler in self.handlers.iter() {
            match handler.invoke(&mut req, res) {
                Ok(Halt(res)) => {
                    debug!("{:?} {:?} {:?} {:?}",
                           req.origin.method,
                           req.origin.remote_addr,
                           req.origin.uri,
                           res.origin.status());
                    return
                }
                Ok(Continue(fresh)) => res = fresh,
                Err(mut err) => {
                    warn!("{:?} {:?} {:?} {:?} {:?} {:?}",
                          req.origin.method,
                          req.origin.remote_addr,
                          req.origin.uri,
                          err.kind,
                          err.message,
                          err.stream.as_ref().map(|s| s.origin.status()));

                    for error_handler in self.error_handlers.iter().rev() {
                        if let Halt(()) = error_handler.handle_error(&mut err, &mut req) {
                            return
                        }
                    }

                    warn!("Unhandled error: {:?} {:?} {:?} {:?} {:?} {:?}",
                          req.origin.method,
                          req.origin.remote_addr,
                          req.origin.uri,
                          err.kind,
                          err.message,
                          err.stream.map(|s| s.origin.status()));
                    return
                }
            }
        }
    }

    pub fn new () -> MiddlewareStack {
        MiddlewareStack{
            handlers: Vec::new(),
            error_handlers: Vec::new()
        }
    }
}
