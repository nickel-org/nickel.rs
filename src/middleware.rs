use crate::request::Request;
use crate::response::Response;
use crate::nickel_error::NickelError;

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<'mw, D= ()> = Result<Action<Response<'mw, D>>,
                                                  NickelError<'mw, D>>;

pub enum Action<T=()> {
    Continue(T),
    Halt(T)
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware<D>: Send + 'static + Sync {
    fn invoke<'mw, 'conn>(&'mw self, _req: &mut Request<'mw, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        res.next_middleware()
    }
}

impl<T, D> Middleware<D> for T where T: for<'r, 'mw, 'conn> Fn(&'r mut Request<'mw, D>, Response<'mw, D>) -> MiddlewareResult<'mw, D> + Send + Sync + 'static {
    fn invoke<'mw, 'conn>(&'mw self, req: &mut Request<'mw, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        (*self)(req, res)
    }
}

pub trait ErrorHandler<D>: Send + 'static + Sync {
    fn handle_error(&self, _: &mut NickelError<'_, D>, _: &mut Request<'_, D>) -> Action;
}

impl<D: 'static> ErrorHandler<D> for fn(&mut NickelError<'_, D>, &mut Request<'_, D>) -> Action {
    fn handle_error(&self, err: &mut NickelError<'_, D>, req: &mut Request<'_, D>) -> Action {
        (*self)(err, req)
    }
}

pub struct MiddlewareStack<D=()> {
    handlers: Vec<Box<dyn Middleware<D> + Send + Sync>>,
    error_handlers: Vec<Box<dyn ErrorHandler<D> + Send + Sync>>
}

impl<D: 'static> MiddlewareStack<D> {
    pub fn add_middleware<T: Middleware<D>> (&mut self, handler: T) {
        self.handlers.push(Box::new(handler));
    }

    pub fn add_error_handler<T: ErrorHandler<D>> (&mut self, handler: T) {
        self.error_handlers.push(Box::new(handler));
    }

    pub fn invoke<'mw, 'conn>(&'mw self, mut req: Request<'mw, D>, mut res: Response<'mw, D>) -> Response<'mw, D> {
        for handler in self.handlers.iter() {
            match handler.invoke(&mut req, res) {
                Ok(Halt(res)) => {
                    debug!("Halted {:?} {:?} {:?} {:?}",
                           req.origin.method(),
                           req.remote_addr(),
                           req.origin.uri(),
                           res.status());
                    // let _ = res.end();
                    return res;
                },
                Ok(Continue(fresh)) => res = fresh,
                Err(mut err) => {
                    warn!("{:?} {:?} {:?} {:?} {:?}",
                          req.origin.method(),
                          req.remote_addr(),
                          req.origin.uri(),
                          err.message,
                          err.stream.as_ref().map(|s| s.status()));

                    for error_handler in self.error_handlers.iter().rev() {
                        if let Halt(()) = error_handler.handle_error(&mut err, &mut req) {
                            if let Some(res) = err.stream {
                                return res;
                            } else {
                                error!("Error without Response struct");
                                // Create a new Response with an InternalServerError

                                panic!("Error without Response struct"); // Todo: migration cleanup - return error
                            }
                        }
                    }

                    warn!("Unhandled Error: {:?} {:?} {:?} {:?} {:?}",
                          req.origin.method(),
                          req.remote_addr(),
                          req.origin.uri(),
                          err.message,
                          err.stream.map(|s| s.status()));
                    panic!("Unhandled Error"); // Todo: migration cleanup - return error
                }
            }
        }
        // No middleware returned Halt, go with the last one in the train
        res
    }

    pub fn new () -> MiddlewareStack<D> {
        MiddlewareStack{
            handlers: Vec::new(),
            error_handlers: Vec::new()
        }
    }
}
