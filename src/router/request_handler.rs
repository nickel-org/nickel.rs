use request::Request;
use response::Response;
use http::status;
use http::headers;
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

impl<'a, S: Show> ResponseFinalizer for &'a [S] {
    fn respond(self, res: &mut Response) {
        res.origin.status = status::Ok;
        for ref s in self.iter() {
            // FIXME : failure unhandled
            let _ = write!(res.origin, "{}", s);
        }
    }
}

macro_rules! dual_impl(
    ($view:ty, $alloc:ty |$s:ident, $res:ident| $b:block) => (
        impl<'a> ResponseFinalizer for $view {
            fn respond($s, $res: &mut Response) $b
        }

        impl ResponseFinalizer for $alloc {
            fn respond($s, $res: &mut Response) $b
        }
    )
)

dual_impl!(&'a str,
           String
            |self, res| {
                res.origin.status = status::Ok;
                res.send(self);
            })

dual_impl!((status::Status, &'a str),
           (status::Status, String)
            |self, res| {
                let (status, data) = self;
                res.origin.status = status;
                res.send(data);
            })

dual_impl!((uint, &'a str),
           (uint, String)
           |self, res| {
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
            })
