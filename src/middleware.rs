use async_trait::async_trait;
use crate::request::Request;
use crate::response::Response;
use crate::nickel_error::NickelError;
use hyper::{Body, Response as HyperResponse};

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<D= ()> = Result<Action<Response<D>>,
                                          NickelError<D>>;

pub enum Action<T=()> {
    Continue(T),
    Halt(T)
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
#[async_trait]
pub trait Middleware<D: Send + 'static + Sync>: Send + 'static + Sync {
    async fn invoke(&self, _req: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
        res.next_middleware()
    }
}

#[async_trait]
impl<T, D> Middleware<D> for T
where T: Fn(&mut Request<D>, Response<D>) -> MiddlewareResult<D> + Send + Sync + 'static,
      D: Send + 'static + Sync
{
    async fn invoke(&self, req: &mut Request<D>, res: Response<D>) -> MiddlewareResult<D> {
        (*self)(req, res)
    }
}

pub trait ErrorHandler<D: Send + 'static + Sync>: Send + 'static + Sync {
    fn handle_error(&self, _: &mut NickelError<D>, _: &mut Request<D>) -> Action;
}

impl<D: Send + 'static + Sync> ErrorHandler<D> for fn(&mut NickelError<D>, &mut Request<D>) -> Action {
    fn handle_error(&self, err: &mut NickelError<D>, req: &mut Request<D>) -> Action {
        (*self)(err, req)
    }
}

pub struct MiddlewareStack<D: Send + 'static + Sync = ()> {
    handlers: Vec<Box<dyn Middleware<D> + Send + Sync>>,
    error_handlers: Vec<Box<dyn ErrorHandler<D> + Send + Sync>>
}

impl<D: Send + 'static + Sync> MiddlewareStack<D> {
    pub fn add_middleware<T: Middleware<D>> (&mut self, handler: T) {
        self.handlers.push(Box::new(handler));
    }

    pub fn add_error_handler<T: ErrorHandler<D>> (&mut self, handler: T) {
        self.error_handlers.push(Box::new(handler));
    }

    pub async fn invoke(&self, mut req: Request<D>, mut res: Response<D>) -> HyperResponse<Body> {
        for handler in self.handlers.iter() {
            match handler.invoke(&mut req, res).await {
                Ok(Halt(res)) => {
                    debug!("Halted {:?} {:?} {:?} {:?}",
                           req.origin.method(),
                           req.remote_addr(),
                           req.origin.uri(),
                           res.status());
                    // let _ = res.end();
                    return res.origin;
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
                                return res.origin;
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
        res.origin
    }

    pub fn new () -> MiddlewareStack<D> {
        MiddlewareStack{
            handlers: Vec::new(),
            error_handlers: Vec::new()
        }
    }
}
