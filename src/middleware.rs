use response::Response;
use nickel_error::NickelError;
use hyper::net;

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<'a, 'k, D> = Result<Action<Response<'a, 'k, D, net::Fresh>,
                                                 Response<'a, 'k, D, net::Streaming>>,
                                          NickelError<'a, 'k, D>>;

pub enum Action<T=(), U=()> {
    Continue(T),
    Halt(U)
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware<D>: Send + 'static + Sync {
    fn invoke<'a, 'k>(&'a self, res: Response<'a, 'k, D, net::Fresh>) -> MiddlewareResult<'a, 'k, D> {
        Ok(Continue(res))
    }
}

impl<T, D> Middleware<D> for T where T: for<'a, 'k> Fn(Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> + Send + Sync + 'static {
    fn invoke<'a, 'k>(&'a self, res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
        (*self)(res)
    }
}

pub trait ErrorHandler<D>: Send + 'static + Sync {
    fn handle_error(&self, &mut NickelError<D>) -> Action;
}

impl<D> ErrorHandler<D> for fn(&mut NickelError<D>) -> Action {
    fn handle_error(&self, err: &mut NickelError<D>) -> Action {
        (*self)(err)
    }
}

pub struct MiddlewareStack<D> {
    handlers: Vec<Box<Middleware<D> + Send + Sync>>,
    error_handlers: Vec<Box<ErrorHandler<D> + Send + Sync>>
}

impl<D> MiddlewareStack<D> {
    pub fn add_middleware<T: Middleware<D>> (&mut self, handler: T) {
        self.handlers.push(Box::new(handler));
    }

    pub fn add_error_handler<T: ErrorHandler<D>> (&mut self, handler: T) {
        self.error_handlers.push(Box::new(handler));
    }

    pub fn invoke<'a, 'k>(&'a self, mut res: Response<'a, 'k, D>) {
        for handler in self.handlers.iter() {
            match handler.invoke(res) {
                Ok(Halt(res)) => {
                    debug!("Halted {:?} {:?} {:?} {:?}",
                           res.request.origin.method,
                           res.request.origin.remote_addr,
                           res.request.origin.uri,
                           res.status());
                    let _ = res.end();
                    return
                }
                Ok(Continue(fresh)) => res = fresh,
                Err(mut err) => {
                    warn!("{:?} {:?} {:?} {:?} {:?}",
                          err.request().origin.method,
                          err.request().origin.remote_addr,
                          err.request().origin.uri,
                          err.message,
                          err.response().as_ref().map(|s| s.status()));

                    for error_handler in self.error_handlers.iter().rev() {
                        if let Halt(()) = error_handler.handle_error(&mut err) {
                           err.end();
                           return
                        }
                    }

                    warn!("Unhandled error: {:?} {:?} {:?} {:?} {:?}",
                          err.request().origin.method,
                          err.request().origin.remote_addr,
                          err.request().origin.uri,
                          err.message,
                          err.response().map(|s| s.status()));
                    return
                }
            }
        }
    }

    pub fn new () -> MiddlewareStack<D> {
        MiddlewareStack{
            handlers: Vec::new(),
            error_handlers: Vec::new()
        }
    }
}
