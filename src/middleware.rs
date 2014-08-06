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
pub trait Middleware: Clone + Send {
    fn invoke (&self, _req: &mut Request, _res: &mut Response) -> Result<Action, NickelError> {
        Ok(Continue)
    }

    // we need this because otherwise clone() would be ambiguous
    fn clone_box(&self) -> Box<Middleware + Send> { 
        box self.clone() as Box<Middleware + Send> 
    }
}

impl Clone for Box<Middleware + Send> {
    fn clone(&self) -> Box<Middleware + Send> {
        self.clone_box()
    }
}

pub trait ErrorHandler: Clone + Send {
    fn invoke (&self, _err: &NickelError, _req: &mut Request, _res: &mut Response) -> Result<Action, NickelError> {
        Ok(Continue)
    }

    // we need this because otherwise clone() would be ambiguous
    fn clone_box(&self) -> Box<ErrorHandler + Send> {
        box self.clone() as Box<ErrorHandler + Send>
    }
}

impl Clone for Box<ErrorHandler + Send> {
    fn clone(&self) -> Box<ErrorHandler + Send> {
        self.clone_box()
    }
}

// this is temporally not possible anymore
// Read https://github.com/iron/iron/issues/76 for more details

// impl Middleware for fn (req: &Request, res: &mut Response) -> bool {
//     fn invoke(&self, req: &mut Request, res: &mut Response) -> bool{
//         (*self)(req, res)
//     }
// }

pub struct FromFn {
    func: fn (req: &Request, res: &mut Response) -> Result<Action, NickelError>
}

impl FromFn {
    pub fn new (func: fn (req: &Request, res: &mut Response) -> Result<Action, NickelError>) -> FromFn {
        FromFn {
            func: func
        }
    }
}

impl Middleware for FromFn {
    fn invoke (&self, req: &mut Request, res: &mut Response) -> Result<Action, NickelError> {
        (self.func)(req, res)
    }
}

impl Clone for FromFn {
    fn clone(&self) -> FromFn {
        *self
    }
}

#[deriving(Clone)]
pub struct MiddlewareStack {
    handlers: Vec<Box<Middleware + Send>>,
    error_handlers: Vec<Box<ErrorHandler + Send>>
}

impl MiddlewareStack {
    pub fn add_middleware<T: Middleware> (&mut self, handler: T) {
        self.handlers.push(box handler);
    }

    pub fn add_error_handler<T: ErrorHandler> (&mut self, handler: T) {
        self.error_handlers.push(box handler);
    }

    pub fn invoke (&self, req: &mut Request, res: &mut Response) {
        self.handlers.iter().all(|handler| {
            match (*handler).invoke(req, res) {
                Ok(Continue) => true,
                Ok(Halt)     => false,
                Err(err)     => {
                    let mut err = err;
                    self.error_handlers.iter().all(|error_handler| {
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
