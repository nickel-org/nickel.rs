//! Blanket impls for Middleware.
//! This is pre-implemented for any function which takes a
//! `Request` and `Response` parameter and returns anything
//! implementing the `ResponseFinalizer` trait. It is also
//! implemented for a tuple of a function and a type `T`.
//! The function must take a `Request`, a `Response` and a
//! `T`, returning anything that implements `ResponseFinalizer`.
//! The data of type `T` will then be shared and available
//! in any request.
//!
//! Please see the examples for usage.

use request::Request;
use response::Response;
use http::status;
use http::headers;
use std::fmt::Show;
use middleware::{Middleware, MiddlewareResult, Halt};
use serialize::json;
use mimes::MediaType;

impl<R> Middleware for fn(&Request, &mut Response) -> R
        where R: ResponseFinalizer {
    fn invoke<'a, 'b>(&self, req: &mut Request<'a, 'b>, res: &mut Response) -> MiddlewareResult {
        let r = (*self)(req, res);
        r.respond(res)
    }
}

impl<T, R> Middleware for (fn(&Request, &mut Response, &T) -> R, T)
        where T: Send + Sync, R: ResponseFinalizer + 'static {
    fn invoke<'a, 'b>(&self, req: &mut Request<'a, 'b>, res: &mut Response) -> MiddlewareResult {
        let (f, ref data) = *self;
        let r = f(req, res, data);
        r.respond(res)
    }
}

impl<R> Middleware for fn(&mut Request, &mut Response) -> R
        where R: ResponseFinalizer {
    fn invoke<'a, 'b>(&self, req: &mut Request<'a, 'b>, res: &mut Response) -> MiddlewareResult {
        let r = (*self)(req, res);
        r.respond(res)
    }
}

impl<T, R> Middleware for (fn(&mut Request, &mut Response, &T) -> R, T)
        where T: Send + Sync, R: ResponseFinalizer + 'static {
    fn invoke<'a, 'b>(&self, req: &mut Request<'a, 'b>, res: &mut Response) -> MiddlewareResult {
        let (f, ref data) = *self;
        let r = f(req, res, data);
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
        maybe_set_type(res, MediaType::Html);
        Ok(Halt)
    }
}

impl ResponseFinalizer for MiddlewareResult {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, MediaType::Html);
        self
    }
}

impl ResponseFinalizer for json::Json {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, MediaType::Json);
        res.send(json::encode(&self));
        Ok(Halt)
    }
}

impl<'a, S: Show> ResponseFinalizer for &'a [S] {
    fn respond(self, res: &mut Response) -> MiddlewareResult {
        maybe_set_type(res, MediaType::Html);
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
                maybe_set_type(res, MediaType::Html);
                res.origin.status = status::Ok;
                res.send(self);
                Ok(Halt)
            })

dual_impl!((status::Status, &'a str),
           (status::Status, String)
            |self, res| {
                maybe_set_type(res, MediaType::Html);
                let (status, data) = self;
                res.origin.status = status;
                res.send(data);
                Ok(Halt)
            })

dual_impl!((uint, &'a str),
           (uint, String)
           |self, res| {
                maybe_set_type(res, MediaType::Html);
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
                maybe_set_type(res, MediaType::Html);
                res.send(data);
                Ok(Halt)
            })

fn maybe_set_type(res: &mut Response, ty: MediaType) {
    if res.origin.headers.content_type.is_none() {
        res.content_type(ty);
    }
}
