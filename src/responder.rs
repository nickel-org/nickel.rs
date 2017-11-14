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
use {Response, ResponseStream, NickelError, MiddlewareResult, Halt};
use hyper::{Body, StatusCode};
use hyper::header;
use serialize::json;
use mimes::MediaType;

/// This trait provides convenience for translating a number
/// of common return types into a `MiddlewareResult` while
/// also modifying the `Response` as required.
///
/// Please see the examples for some uses.
pub trait Responder<D> {
    fn respond<'a>(self, Response<'a, D>) -> MiddlewareResult<'a, D>;
}

impl<D> Responder<D> for () {
    fn respond<'a>(self, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        res.next_middleware()
    }
}

impl<D> Responder<D> for json::Json {
    fn respond<'a>(self, mut res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        maybe_set_type(&mut res, MediaType::Json);
        res.send(json::encode(&self)
                      .map_err(|e| format!("Failed to parse JSON: {}", e)))
    }
}

impl<T, E, D> Responder<D> for Result<T, E>
        where T: Responder<D>,
              for<'e> NickelError<'e, D>: From<(Response<'e, D>, E)> {
    fn respond<'a>(self, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        let data = try_with!(res, self);
        res.send(data)
    }
}

impl <'a, D> Responder<D> for &'a [u8] {
    #[allow(unused_mut)]
    #[inline]
    fn respond<'c>(self, mut res: Response<'c, D>) -> MiddlewareResult<'c, D> {
        self.to_vec().respond(res)
    }
}

impl <'a, D> Responder<D> for Vec<u8> {
    #[allow(unused_mut)]
    #[inline]
    fn respond<'c>(self, mut res: Response<'c, D>) -> MiddlewareResult<'c, D> {
        maybe_set_type(&mut res, MediaType::Bin);

        res.start();
        let body: ResponseStream = Box::new(Body::from(self));
        res.origin.set_body(body);
        Ok(Halt(res))
    }
}

macro_rules! dual_impl {
    ($view:ty, $alloc:ty, |$s:ident, $res:ident| $b:block) => (
        impl<'a, D> Responder<D> for $view {
            #[allow(unused_mut)]
            #[inline]
            fn respond<'c>($s, mut $res: Response<'c, D>) -> MiddlewareResult<'c, D> $b
        }

        impl<'a, D> Responder<D> for $alloc {
            #[allow(unused_mut)]
            #[inline]
            fn respond<'c>($s, mut $res: Response<'c, D>) -> MiddlewareResult<'c, D> $b
        }
    )
}

dual_impl!(&'a str,
           String,
            |self, res| {
                maybe_set_type(&mut res, MediaType::Html);
                res.send(self.as_bytes())
            });

dual_impl!((StatusCode, &'static str),
           (StatusCode, String),
            |self, res| {
                let (status, message) = self;

                if status.is_client_error() | status.is_server_error() {
                    res.error(status, message)
                } else {
                    res.set(status);
                    res.send(message)
                }
            });

impl<'a, D> Responder<D> for StatusCode {
    #[inline]
    fn respond<'c>(self, res: Response<'c, D>) -> MiddlewareResult<'c, D> {
        res.send((self, ""))
    }
}

dual_impl!(&'a [&'a str],
           &'a [String],
            |self, res| {
                self.iter().fold("".to_string(), |a, s| {a + s}).respond(res)
            });

dual_impl!((u16, &'static str),
           (u16, String),
           |self, res| {
                let (status, message) = self;
                res.send((StatusCode::try_from(status).unwrap_or(StatusCode::InternalServerError), message))
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

fn maybe_set_type<D>(res: &mut Response<D>, mime: MediaType) {
    res.set_header_fallback(|| header::ContentType(mime.into()));
}
