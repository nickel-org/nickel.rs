use request::Request;
use response::Response;

// the usage of + Send is weird here because what we really want is + Static
// but that's not possible as of today. We have to use + Send for now.
pub trait Middleware: Clone + Send {
    fn invoke (&self, _req: &mut Request, _res: &mut Response) -> bool {
        true
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

// this is temporally not possible anymore
// Read https://github.com/iron/iron/issues/76 for more details

// impl Middleware for fn (req: &Request, res: &mut Response) -> bool {
//     fn invoke(&self, req: &mut Request, res: &mut Response) -> bool{
//         (*self)(req, res)
//     }
// }

pub struct FromFn {
    func: fn (req: &Request, res: &mut Response) -> bool
}

impl FromFn {
    pub fn new (func: fn (req: &Request, res: &mut Response) -> bool) -> FromFn {
        FromFn {
            func: func
        }
    }
}

impl Middleware for FromFn {
    fn invoke (&self, req: &mut Request, res: &mut Response) -> bool{
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
    handlers: Vec<Box<Middleware + Send>>
}

impl MiddlewareStack {
    pub fn add<T: Middleware> (&mut self, handler: T) {
        self.handlers.push(box handler);
    }

    pub fn invoke (&self, req: &mut Request, res: &mut Response) {
        self.handlers.iter().all(|handler| (*handler).invoke(req, res));
    }

    pub fn new () -> MiddlewareStack {
        MiddlewareStack{
            handlers: Vec::new()
        }
    }
}