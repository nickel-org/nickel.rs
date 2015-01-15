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
use hyper::status::StatusCode;
use std::fmt::Display;
use std::num::FromPrimitive;
use hyper::header;
use hyper::net;
use middleware::{Middleware, MiddlewareResult, Halt, Continue};
use serialize::json;
use mimes::MediaType;

impl<R> Middleware for fn(&Request, &mut Response) -> R
        where R: ResponseFinalizer {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        let r = (*self)(req, &mut res);
        r.respond(res)
    }
}

impl<T, R> Middleware for (fn(&Request, &mut Response, &T) -> R, T)
        where T: Send + 'static + Sync, R: ResponseFinalizer + 'static {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        let (f, ref data) = *self;
        let r = f(req, &mut res, data);
        r.respond(res)
    }
}

impl<R> Middleware for fn(&mut Request, &mut Response) -> R
        where R: ResponseFinalizer {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        let r = (*self)(req, &mut res);
        r.respond(res)
    }
}

impl<T, R> Middleware for (fn(&mut Request, &mut Response, &T) -> R, T)
        where T: Send + Sync + 'static, R: ResponseFinalizer + 'static {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        let (f, ref data) = *self;
        let r = f(req, &mut res, data);
        r.respond(res)
    }
}

/// This trait provides convenience for translating a number
/// of common return types into a `MiddlewareResult` while
/// also modifying the `Response` as required.
///
/// Please see the examples for some uses.
pub trait ResponseFinalizer<T=net::Fresh> {
    fn respond<'a, 'b>(self, Response<'a, 'a, T>) -> MiddlewareResult<'a, 'a>;
}

impl ResponseFinalizer for () {
    fn respond<'a, 'b>(self, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        maybe_set_type(&mut res, MediaType::Html);
        Ok(Halt(try!(res.start())))
    }
}

// This is impossible?
// impl<'a, 'b> ResponseFinalizer for MiddlewareResult<'a, 'b> {
//     fn respond<'a, 'b>(self, res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
//         maybe_set_type(&mut res, MediaType::Html);
//         self
//     }
// }

impl ResponseFinalizer for json::Json {
    fn respond<'a, 'b>(self, mut res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
        maybe_set_type(&mut res, MediaType::Json);
        // let mut stream = try!(res.start());
        // try!(stream.write(json::encode(&self).as_bytes()));
        let mut stream = try!(res.send(json::encode(&self)));
        Ok(Halt(stream))
    }
}

impl<'a, S: Display> ResponseFinalizer for &'a [S] {
    fn respond<'c, 'b>(self, mut res: Response<'c, 'c>) -> MiddlewareResult<'c, 'c> {
        maybe_set_type(&mut res, MediaType::Html);
        res.status_code(StatusCode::Ok);
        let mut res = try!(res.start());
        for ref s in self.iter() {
            // FIXME : failure unhandled
            let _ = write!(&mut res, "{}", s);
        }
        Ok(Halt(res))
    }
}

macro_rules! dual_impl {
    ($view:ty, $alloc:ty, |$s:ident, $res:ident| $b:block) => (
        impl<'a> ResponseFinalizer for $view {
            fn respond<'a, 'b>($s, mut $res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> $b
        }

        impl ResponseFinalizer for $alloc {
            fn respond<'a, 'b>($s, mut $res: Response<'a, 'a>) -> MiddlewareResult<'a, 'a> $b
        }
    )
}

dual_impl!(&'a str,
           String,
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);

                res.status_code(StatusCode::Ok);
                let stream = try!(res.send(self));
                Ok(Halt(stream))
            });

dual_impl!((StatusCode, &'a str),
           (StatusCode, String),
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                let (status, data) = self;

                res.status_code(status);
                let stream = try!(res.send(data));
                Ok(Halt(stream))
            });

dual_impl!((usize, &'a str),
           (usize, String),
           |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                let (status, data) = self;
                match FromPrimitive::from_uint(status) {
                    Some(status) => {
                        res.status_code(status);
                        let stream = try!(res.send(data));
                        Ok(Halt(stream))
                    }
                    // This is a logic error
                    None => panic!("Bad status code")
                }
            });

// FIXME: Hyper uses traits for headers, so this needs to be a Vec of
// trait objects. But, a trait object is unable to have Foo + Bar as a bound.
//
// A better/faster solution would be to impl this for tuples,
// where each tuple element implements the Header trait, which would give a
// static dispatch.
// dual_impl!((StatusCode, &'a str, Vec<Box<ResponseHeader>>),
//            (StatusCode, String, Vec<Box<ResponseHeader>>)
//            |self, res| {
//                 let (status, data, headers) = self;

//                 res.origin.status = status;
//                 for header in headers.into_iter() {
//                     res.origin.headers_mut().set(header);
//                 }
//                 maybe_set_type(&mut res, MediaType::Html);
//                 res.send(data);
//                 Ok(Halt)
//             })

fn maybe_set_type(res: &mut Response, ty: MediaType) {
    if res.origin.headers().has::<header::ContentType>() {
        res.content_type(ty);
    }
}
