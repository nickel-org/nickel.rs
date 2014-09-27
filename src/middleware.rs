use request::Request;
use response::Response;
use nickel_error::NickelError;

#[deriving(PartialEq)]
pub enum Action {
  Continue,
  Halt
}

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Send + Sync {
    fn invoke<'a, 'b>(&'a self, _req: &mut Request<'b, 'a>, _res: &mut Response) -> Result<Action, NickelError> {
        Ok(Continue)
    }
}

pub trait ErrorHandler: Send + Sync {
    fn invoke(&self, _err: &NickelError, _req: &mut Request, _res: &mut Response) -> Result<Action, NickelError> {
        Ok(Continue)
    }
}

// this is temporally not possible anymore
// Read https://github.com/iron/iron/issues/76 for more details

// impl Middleware for fn (req: &Request, res: &mut Response) -> Result<Action, NickelError> {
//     fn invoke(&self, req: &mut Request, res: &mut Response) -> Result<Action, NickelError> {
//         (*self)(req, res)
//     }
// }

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
        self.handlers.iter().all(|handler| {
            match (*handler).invoke(req, res) {
                Ok(Continue) => true,
                Ok(Halt)     => false,
                Err(err)     => {
                    let mut err = err;
                    self.error_handlers.iter().rev().all(|error_handler| {
                        match (*error_handler).invoke(&err, req, res) {
                            Ok(Continue) => true,
                            Ok(Halt)     => false,
                            Err(new_err)     => {
                                // change the error so that other ErrorHandler down the stack receive
                                // the new error.
                                err = new_err;
                                true
                            }
                        }
                    })
                }
            }
        });
    }

    pub fn new () -> MiddlewareStack {
        MiddlewareStack{
            handlers: Vec::new(),
            error_handlers: Vec::new()
        }
    }
}
