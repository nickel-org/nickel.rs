use request::Request;
use response::Response;
use http::status;
use http::headers;
use std::fmt::Show;
use middleware::{MiddlewareResult, Halt};
use serialize::json;
use mimes::get_media_type;

/// Handles a HTTP request
/// This is pre-implemented for any function which takes a
/// `Request` and `Response` parameter and returns anything
/// implementing the `ResponseFinalizer` trait.
///
/// Please see the examples for usage.
pub trait RequestHandler<T: Send + Sync> : Sync + Send {
    fn handle(&self, &Request, &mut Response, &T) -> MiddlewareResult;
}

impl<R, T: Send + Sync> RequestHandler<T> for fn(request: &Request, response: &mut Response) -> R
        where R: ResponseFinalizer {
    fn handle(&self, req: &Request, res: &mut Response, _: &T) -> MiddlewareResult {
        let r = (*self)(req, res);
        r.respond(res)
    }
}

impl<R, T: Send + Sync> RequestHandler<T> for fn(request: &Request, response: &mut Response, route_data: &T) -> R
        where R: ResponseFinalizer {
    fn handle(&self, req: &Request, res: &mut Response, route_data: &T) -> MiddlewareResult {
        let r = (*self)(req, res, route_data);
        r.respond(res)
    }

}

/// This trait provides convenience for translating a number
/// of common return types into a `MiddlewareResult` while
/// also modifying the `Response` as required.
///
/// Please see the examples for some uses.
pub trait ResponseFinalizer {
    fn respond(self, &mut Response) -> MiddlewareResult;
}

impl ResponseFinalizer for () {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, "html");
        Ok(Halt)
    }
}

impl ResponseFinalizer for MiddlewareResult {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, "html");
        self
    }
}

impl ResponseFinalizer for json::Json {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, "json");
        res.send(json::encode(&self));
        Ok(Halt)
    }
}

impl<'a, S: Show> ResponseFinalizer for &'a [S] {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, "html");
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
                maybe_set_type(res, "html");
                res.origin.status = status::Ok;
                res.send(self);
                Ok(Halt)
            })

dual_impl!((status::Status, &'a str),
           (status::Status, String)
            |self, res| {
                maybe_set_type(res, "html");
                let (status, data) = self;
                res.origin.status = status;
                res.send(data);
                Ok(Halt)
            })

dual_impl!((uint, &'a str),
           (uint, String)
           |self, res| {
                maybe_set_type(res, "html");
                let (status, data) = self;
                match FromPrimitive::from_uint(status) {
                    Some(status) => {
                        res.origin.status = status;
                        res.send(data);
                        Ok(Halt)
                    }
                    // This is a logic error
                    None => panic!("Bad status code")
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
                maybe_set_type(res, "html");
                res.send(data);
                Ok(Halt)
            })

fn maybe_set_type(res: &mut Response, ty: &str) {
    if res.origin.headers.content_type.is_none() {
        res.origin.headers.content_type = get_media_type(ty);
    }
}
