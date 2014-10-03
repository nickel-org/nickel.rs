use request::Request;
use response::Response;
use http::status;
use std::fmt::Show;

pub trait RequestHandler : Sync + Send {
    fn handle(&self, &Request, &mut Response);
}

impl<R> RequestHandler for fn(request: &Request, response: &mut Response) -> R
        where R: ResponseFinalizer {
    fn handle(&self, req: &Request, res: &mut Response) {
        let r = (*self)(req, res);
        r.respond(res);
    }
}

pub trait ResponseFinalizer {
    fn respond(self, &mut Response);
}

impl ResponseFinalizer for () {
    fn respond(self, _: &mut Response) {}
}

impl ResponseFinalizer for String {
    fn respond(self, res: &mut Response) {
        res.origin.status = status::Ok;
        res.send(self);
    }
}

impl<'a> ResponseFinalizer for &'a str {
    fn respond(self, res: &mut Response) {
        res.origin.status = status::Ok;
        res.send(self);
    }
}

impl<'a, S: Show> ResponseFinalizer for &'a [S] {
    fn respond(self, res: &mut Response) {
        res.origin.status = status::Ok;
        for ref s in self.iter() {
            // FIXME : failure unhandled
            let _ = write!(res.origin, "{}", s);
        }
    }
}

impl ResponseFinalizer for (status::Status, String) {
    fn respond(self, res: &mut Response) {
        let (status, data) = self;
        res.origin.status = status;
        res.send(data);
    }
}

impl<'a> ResponseFinalizer for (status::Status, &'a str) {
    fn respond(self, res: &mut Response) {
        let (status, data) = self;
        res.origin.status = status;
        res.send(data);
    }
}

impl<'a> ResponseFinalizer for (uint, String) {
    fn respond(self, res: &mut Response) {
        let (status, data) = self;
        match FromPrimitive::from_uint(status) {
            Some(status) => {
                res.origin.status = status;
                res.send(data);
            }
            None => {
                res.origin.status = status::InternalServerError;
                res.send("ERROR") //FIXME
            }
        }
    }
}

impl<'a> ResponseFinalizer for (uint, &'a str) {
    fn respond(self, res: &mut Response) {
        let (status, data) = self;
        match FromPrimitive::from_uint(status) {
            Some(status) => {
                res.origin.status = status;
                res.send(data);
            }
            None => {
                res.origin.status = status::InternalServerError;
                res.send("ERROR") //FIXME
            }
        }
    }
}
