use request::Request;
use response::Response;

pub trait RequestHandler : Sync + Send {
    fn handle(&self, &Request, &mut Response);
}

impl RequestHandler for fn(request: &Request, response: &mut Response) {
    fn handle(&self, req: &Request, res: &mut Response) {
        (*self)(req, res)
    }
}
