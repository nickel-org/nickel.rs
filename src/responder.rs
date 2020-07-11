//! Blanket impls for Middleware.
//! This is pre-implemented for any function which takes a
//! `Request` and `Response` parameter and returns anything
//! implementing the `Responder` trait. It is also
//! implemented for a tuple of a function and a type `T`.
//! The function must take a `Request`, a `Response` and a
//! `T`, returning anything that implements `Responder`.
//! The data of type `T` will then be shared and available
//! in any request.
//!
//! Please see the examples for usage.
use crate::{Response, NickelError, MiddlewareResult, Halt};
use hyper::{Body, StatusCode};
use hyper::header::{self, HeaderValue};
use mime::Mime;
use serde_json;
use crate::mimes::MediaType;
use std::io::Write;

/// This trait provides convenience for translating a number
/// of common return types into a `MiddlewareResult` while
/// also modifying the `Response` as required.
///
/// Please see the examples for some uses.
pub trait Responder<D: Send + 'static + Sync> {
    fn respond(self, _: Response<D>) -> MiddlewareResult<D>;
}

impl<D: Send + 'static + Sync> Responder<D> for () {
    fn respond(self, res: Response<D>) -> MiddlewareResult<D> {
        res.next_middleware()
    }
}

impl<D: Send + 'static + Sync> Responder<D> for serde_json::Value {
    fn respond(self, mut res: Response<D>) -> MiddlewareResult<D> {
        maybe_set_type(&mut res, MediaType::Json);
        res.send(serde_json::to_string(&self)
                      .map_err(|e| format!("Failed to parse JSON: {}", e)))
    }
}

impl<T, E, D: Send + 'static + Sync> Responder<D> for Result<T, E>
where T: Responder<D>,
for<> NickelError<D>: From<(Response<D>, E)> {
    fn respond(self, res: Response<D>) -> MiddlewareResult<D> {
        let data = try_with!(res, self);
        res.send(data)
    }
}

impl <D: Send + 'static + Sync> Responder<D> for &[u8] {
    #[allow(unused_mut)]
    #[inline]
    fn respond(self, mut res: Response<D>) -> MiddlewareResult<D> {
        // this may be inefficient, copies int a Vec
        self.to_vec().respond(res)
    }
}

impl <D: Send + 'static + Sync> Responder<D> for Vec<u8> {
    #[allow(unused_mut)]
    #[inline]
    fn respond(self, mut res: Response<D>) -> MiddlewareResult<D> {
        maybe_set_type(&mut res, MediaType::Bin);

        res.start();
        res.set_body(self);
        Ok(Halt(res))
    }
}

macro_rules! dual_impl {
    ($view:ty, $alloc:ty, |$s:ident, $res:ident| $b:block) => (
        impl<D: Send + 'static + Sync> Responder<D> for $view {
            #[allow(unused_mut)]
            #[inline]
            fn respond<'c>($s, mut $res: Response<D>) -> MiddlewareResult<D> $b
        }

        impl<D: Send + 'static + Sync> Responder<D> for $alloc {
            #[allow(unused_mut)]
            #[inline]
            fn respond<'c>($s, mut $res: Response<D>) -> MiddlewareResult<D> $b
        }
    )
}

dual_impl!(&str,
           String,
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                res.send(self.as_bytes())
            });

dual_impl!((StatusCode, &'static str),
           (StatusCode, String),
            |self, res| {
                let (status, message) = self;
                let status_code = status.as_u16();
                if status_code >= 400 && status_code <= 599 {
                    res.error(status, message)
                } else {
                    res.set(status);
                    res.send(message)
                }                    
            });

impl<D: Send + 'static + Sync> Responder<D> for StatusCode {
    #[inline]
    fn respond<'c>(self, res: Response<D>) -> MiddlewareResult<D> {
        res.send((self, ""))
    }
}

dual_impl!(&[&str],
           &[String],
           |self, res| {
               // this may be inefficient, copies everything to one String
               self.iter().fold("".to_string(), |a, s| {a + s}).respond(res)
            });

dual_impl!((u16, &'static str),
           (u16, String),
           |self, res| {
               let (status_u16, message) = self;
               let status = StatusCode::from_u16(status_u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
               res.send((status, message))
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

fn maybe_set_type<D: Send + 'static + Sync>(res: &mut Response<D>, media_type: MediaType) {
    let value: HeaderValue = media_type.into();
    res.set_header_fallback(&header::CONTENT_TYPE, &media_type.into());
}
