use request::Request;
use response::Response;
use nickel_error::NickelError;

pub type MiddlewareResult = Result<Action, NickelError>;

#[deriving(PartialEq)]
pub enum Action {
  Continue,
  Halt
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Send + Sync {
    fn invoke<'a, 'b>(&'a self, _req: &mut Request<'b, 'a>, _res: &mut Response) -> MiddlewareResult {
        Ok(Continue)
    }
}

pub trait ErrorHandler: Send + Sync {
    fn invoke(&self, _err: &NickelError, _req: &mut Request, _res: &mut Response) -> MiddlewareResult {
        Ok(Continue)
    }
}

impl Middleware for fn(&Request, &mut Response) -> MiddlewareResult {
    fn invoke(&self, req: &mut Request, res: &mut Response) -> MiddlewareResult {
        (*self)(req, res)
    }
}

impl ErrorHandler for fn(&NickelError, &Request, &mut Response) -> MiddlewareResult {
    fn invoke(&self, err: &NickelError, req: &mut Request, res: &mut Response) -> MiddlewareResult {
        (*self)(err, req, res)
    }
}

pub struct MiddlewareStack {
    handlers: Vec<Box<Middleware + Send + Sync>>,
    error_handlers: Vec<Box<ErrorHandler + Send + Sync>>
}

impl MiddlewareStack {
    pub fn add_middleware<T: Middleware> (&mut self, handler: T) {
        self.handlers.push(box handler);
    }

    pub fn add_error_handler<T: ErrorHandler> (&mut self, handler: T) {
        self.error_handlers.push(box handler);
    }

    pub fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, res: &mut Response) {
        for handler in self.handlers.iter() {
            match handler.invoke(req, res) {
                Ok(Halt) => 
                {
                    debug!("{} {} {} {}", req.origin.method, req.origin.remote_addr, req.origin.request_uri, res.origin.status);
                    return
                }
                Ok(Continue) => {},
                Err(mut err) => {
                    warn!("{} {} {} {}", req.origin.method, req.origin.remote_addr, req.origin.request_uri, err.kind);
                    for error_handler in self.error_handlers.iter().rev() {
                        match error_handler.invoke(&err, req, res) {
                            Ok(Continue) => {},
                            Ok(Halt) => return,
                            // change the error so that other ErrorHandler
                            // down the stack receive the new error.
                            Err(new_err) => err = new_err,
                        }
                    }
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
