use request::Request;
use response::Response;
use nickel_error::NickelError;
use middleware::{ Middleware, Action };

pub struct IntoMiddleware {
    func: fn (req: &Request, res: &mut Response) -> Result<Action, NickelError>
}

impl IntoMiddleware {
    pub fn from_fn (func: fn (req: &Request, res: &mut Response) -> Result<Action, NickelError>) -> IntoMiddleware {
        IntoMiddleware {
            func: func
        }
    }
}

impl Middleware for IntoMiddleware {
    fn invoke (&self, req: &mut Request, res: &mut Response) -> Result<Action, NickelError> {
        (self.func)(req, res)
    }
}

impl Clone for IntoMiddleware {
    fn clone(&self) -> IntoMiddleware {
        *self
    }
}