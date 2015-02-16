use request::Request;
use response::Response;
use nickel_error::NickelError;
use middleware_handler::ResponseFinalizer;
use hyper::net;

pub use self::Action::{Continue, Halt};

pub type MiddlewareResult<'a, 'b> = Result<Action<'a, 'b>, NickelError>;

pub enum Action<'a, 'b: 'a> {
    Continue(Response<'a, 'b, net::Fresh>),
    Halt(Response<'a, 'b, net::Streaming>)
    // TODO: Possibly add a Finished/Handled state here
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Send + 'static + Sync {
    fn invoke<'a, 'b>(&'a self, _req: &mut Request<'b, 'a>, res: Response<'a, 'a, net::Fresh>) -> MiddlewareResult<'a, 'a> {
        Ok(Continue(res))
    }
}

pub trait ErrorHandler: Send + 'static + Sync {
    fn invoke<'a, 'b>(&self, _err: &NickelError, _req: &mut Request, res: Response<'a, 'a, net::Fresh>) -> MiddlewareResult<'a, 'a> {
        Ok(Continue(res))
    }
}

impl<R> ErrorHandler for fn(&NickelError, &Request, &mut Response) -> R
        where R: ResponseFinalizer {
    fn invoke<'a, 'b>(&self, err: &NickelError, req: &mut Request, mut res: Response<'a, 'a, net::Fresh>) -> MiddlewareResult<'a, 'a> {
        let r = (*self)(err, req, &mut res);
        r.respond(res)
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
                Err(err) => panic!("ERROR: {:?}", err),
                // Err(mut err) => {
                //     warn!("{:?} {:?} {:?} {:?}",
                //           req.origin.method,
                //           req.origin.remote_addr,
                //           req.origin.uri,
                //           err.kind);
                //     for error_handler in self.error_handlers.iter().rev() {
                //         match error_handler.invoke(&err, req, res) {
                //             Ok(Continue(fresh)) => res = fresh,
                //             Ok(Halt(_)) => return,
                //             // change the error so that other ErrorHandler
                //             // down the stack receive the new error.
                //             Err(new_err) => err = new_err,
                //         }
                //     }
                // }
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
