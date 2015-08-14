use request::Request;
use response::Response;
use nickel_error::NickelError;
use hyper::net;

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<'mw> = Result<Action<Response<'mw, net::Fresh>,
                                              Response<'mw, net::Streaming>>,
                                        NickelError<'mw>>;

pub enum Action<T=(), U=()> {
    Continue(T),
    Halt(U)
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Send + 'static + Sync {
    fn invoke<'mw, 'conn>(&'mw self, _req: &mut Request<'mw, 'conn>, res: Response<'mw, net::Fresh>) -> MiddlewareResult<'mw> {
        Ok(Continue(res))
    }
}

impl<T> Middleware for T where T: for<'r, 'mw, 'conn> Fn(&'r mut Request<'mw, 'conn>, Response<'mw>) -> MiddlewareResult<'mw> + Send + Sync + 'static {
    fn invoke<'mw, 'conn>(&'mw self, req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
        (*self)(req, res)
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

    pub fn invoke<'mw, 'conn>(&'mw self, mut req: Request<'mw, 'conn>, mut res: Response<'mw>) {
        for handler in self.handlers.iter() {
            match handler.invoke(&mut req, res) {
                Ok(Halt(res)) => {
                    debug!("Halted {:?} {:?} {:?} {:?}",
                           req.origin.method,
                           req.origin.remote_addr,
                           req.origin.uri,
                           res.status());
                    let _ = res.end();
                    return
                }
                Ok(Continue(fresh)) => res = fresh,
                Err(mut err) => {
                    warn!("{:?} {:?} {:?} {:?} {:?}",
                          req.origin.method,
                          req.origin.remote_addr,
                          req.origin.uri,
                          err.message,
                          err.stream.as_ref().map(|s| s.status()));

                    for error_handler in self.error_handlers.iter().rev() {
                        if let Halt(()) = error_handler.handle_error(&mut err, &mut req) {
                            err.end();
                            return
                        }
                    }

                    warn!("Unhandled error: {:?} {:?} {:?} {:?} {:?}",
                          req.origin.method,
                          req.origin.remote_addr,
                          req.origin.uri,
                          err.message,
                          err.stream.map(|s| s.status()));
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
