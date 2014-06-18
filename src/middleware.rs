use request::Request;
use response::Response;

#[deriving(Clone)]
pub struct Middleware {
    handlers: Vec<fn (req: &Request, res: &mut Response) ->bool>
}

impl Middleware {
    pub fn add (&mut self, handler: fn (req: &Request, res: &mut Response) -> bool) {
        self.handlers.push(handler);
    }

    pub fn invoke (&self, req: &Request, res: &mut Response) {
        self.handlers.iter().advance(|handler| (*handler)(req, res));
    }

    pub fn new () -> Middleware {
        Middleware{
            handlers: Vec::new()
        }
    }
}