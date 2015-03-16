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
use mimes::{MediaType, get_media_type};
use std::io::Write;

impl Middleware for for<'a> fn(&mut Request, Response<'a>) -> MiddlewareResult<'a> {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, res: Response<'a>) -> MiddlewareResult<'a> {
        (*self)(req, res)
    }
}

impl<T> Middleware for (for <'a> fn(&mut Request, Response<'a>, &T) -> MiddlewareResult<'a>, T)
        where T: Send + Sync + 'static {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, res: Response<'a>) -> MiddlewareResult<'a> {
        let (f, ref data) = *self;
        f(req, res, data)
    }
}

/// This trait provides convenience for translating a number
/// of common return types into a `MiddlewareResult` while
/// also modifying the `Response` as required.
///
/// Please see the examples for some uses.
pub trait ResponseFinalizer<T=net::Fresh> {
    fn respond<'a>(self, Response<'a, T>) -> MiddlewareResult<'a>;
}

impl ResponseFinalizer for () {
    fn respond<'a>(self, res: Response<'a>) -> MiddlewareResult<'a> {
        Ok(Continue(res))
    }
}

// This is impossible?
// impl<'a> ResponseFinalizer for MiddlewareResult<'a> {
//     fn respond<'a>(self, res: Response<'a>) -> MiddlewareResult<'a> {
//         maybe_set_type(&mut res, MediaType::Html);
//         self
//     }
// }

impl ResponseFinalizer for json::Json {
    fn respond<'a>(self, mut res: Response<'a>) -> MiddlewareResult<'a> {
        maybe_set_type(&mut res, MediaType::Json);
        res.send(json::encode(&self).unwrap())
    }
}

impl<'a, S: Display> ResponseFinalizer for &'a [S] {
    fn respond<'c>(self, mut res: Response<'c>) -> MiddlewareResult<'c> {
        maybe_set_type(&mut res, MediaType::Html);
        res.set_status(StatusCode::Ok);
        let mut stream = try!(res.start());
        for ref s in self.iter() {
            // FIXME : This error handling is poor
            match stream.write_fmt(format_args!("{}", s)) {
                Ok(()) => {},
                Err(e) => return stream.bail(format!("Failed to write to stream: {}", e))
            }
        }
        Ok(Halt(stream))
    }
}

macro_rules! dual_impl {
    ($view:ty, $alloc:ty, |$s:ident, $res:ident| $b:block) => (
        impl<'a> ResponseFinalizer for $view {
            fn respond<'c>($s, mut $res: Response<'c>) -> MiddlewareResult<'c> $b
        }

        impl ResponseFinalizer for $alloc {
            fn respond<'c>($s, mut $res: Response<'c>) -> MiddlewareResult<'c> $b
        }
    )
}

dual_impl!(&'a str,
           String,
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);

                res.set_status(StatusCode::Ok);
                res.send(self)
            });

dual_impl!((StatusCode, &'a str),
           (StatusCode, String),
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                let (status, data) = self;

                res.set_status(status);
                res.send(data)
            });

dual_impl!((usize, &'a str),
           (usize, String),
           |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                let (status, data) = self;
                match FromPrimitive::from_usize(status) {
                    Some(status) => {
                        res.set_status(status);
                        res.send(data)
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
    res.set_header_fallback(|| header::ContentType(get_media_type(ty)));
}
