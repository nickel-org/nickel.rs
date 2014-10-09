use request::Request;
use response::Response;
use http::status;
use http::headers;
use std::fmt::Show;
use middleware::{MiddlewareResult, Halt};

pub trait RequestHandler : Sync + Send {
    fn handle(&self, &Request, &mut Response) -> MiddlewareResult;
}

impl<R> RequestHandler for fn(request: &Request, response: &mut Response) -> R
        where R: ResponseFinalizer {
    fn handle(&self, req: &Request, res: &mut Response) -> MiddlewareResult {
        let r = (*self)(req, res);
        r.respond(res)
    }
}

pub trait ResponseFinalizer {
    fn respond(self, &mut Response) -> MiddlewareResult;
}

impl ResponseFinalizer for () {
    fn respond(self, _: &mut Response) -> MiddlewareResult {
        Ok(Halt)
    }
}

impl ResponseFinalizer for MiddlewareResult {
    fn respond(self, _res: &mut Response) -> MiddlewareResult {
        self
    }
}

impl<'a, S: Show> ResponseFinalizer for &'a [S] {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        res.origin.status = status::Ok;
        for ref s in self.iter() {
            // FIXME : failure unhandled
            let _ = write!(res.origin, "{}", s);
        }
        Ok(Halt)
    }
}

macro_rules! dual_impl(
    ($view:ty, $alloc:ty |$s:ident, $res:ident| $b:block) => (
        impl<'a> ResponseFinalizer for $view {
            fn respond($s, $res: &mut Response) -> MiddlewareResult $b
        }

        impl ResponseFinalizer for $alloc {
            fn respond($s, $res: &mut Response) -> MiddlewareResult $b
        }
    )
)

dual_impl!(&'a str,
           String
            |self, res| {
                res.origin.status = status::Ok;
                res.send(self);
                Ok(Halt)
            })

dual_impl!((status::Status, &'a str),
           (status::Status, String)
            |self, res| {
                let (status, data) = self;
                res.origin.status = status;
                res.send(data);
                Ok(Halt)
            })

dual_impl!((uint, &'a str),
           (uint, String)
           |self, res| {
                let (status, data) = self;
                match FromPrimitive::from_uint(status) {
                    Some(status) => {
                        res.origin.status = status;
                        res.send(data);
                        Ok(Halt)
                    }
                    // This is a logic error
                    None => fail!("Bad status code")
                }
            })

dual_impl!((status::Status, &'a str, Vec<headers::response::Header>),
           (status::Status, String, Vec<headers::response::Header>)
           |self, res| {
                let (status, data, headers) = self;

                res.origin.status = status;
                for header in headers.into_iter() {
                    res.origin.headers.insert(header);
                }
                res.send(data);
                Ok(Halt)
            })
