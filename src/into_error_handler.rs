use request::Request;
use response::Response;
use nickel_error::NickelError;
use middleware::{ ErrorHandler, Action };

pub struct IntoErrorHandler {
    func: fn (err: &NickelError, req: &Request, res: &mut Response) -> Result<Action, NickelError>
}

impl IntoErrorHandler {
    pub fn from_fn (func: fn (err: &NickelError, req: &Request, res: &mut Response) -> Result<Action, NickelError>) -> IntoErrorHandler {
        IntoErrorHandler {
            func: func
        }
    }
}

impl ErrorHandler for IntoErrorHandler {
    fn invoke (&self, err: &NickelError, req: &mut Request, res: &mut Response) -> Result<Action, NickelError> {
        (self.func)(err, req, res)
    }
}

impl Clone for IntoErrorHandler {
    fn clone(&self) -> IntoErrorHandler {
        *self
    }
}