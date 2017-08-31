/// Macro to reduce the boilerplate required for using unboxed
/// closures as `Middleware` due to current type inference behaviour.
///
/// In future, the macro should hopefully be able to be removed while
/// having minimal changes to the closure's code.
///
/// # Examples
/// ```rust,no_run
/// # #[macro_use] extern crate nickel;
/// # fn main() {
/// use nickel::{Nickel, HttpRouter};
/// use std::sync::atomic::{AtomicUsize, Ordering};
///
/// let mut server = Nickel::new();
///
/// // Some shared resource between requests, must be `Sync + Send`
/// let visits = AtomicUsize::new(0);
///
/// server.get("/", middleware! {
///     format!("{}", visits.fetch_add(1, Ordering::Relaxed))
/// });
///
/// server.listen("127.0.0.1:6767").unwrap().wait();
/// # }
/// ```
///
/// # Type hinting
/// Sometimes type inference is unable to determine the datatype for the server,
/// which can lead to a lot of extra type annotations. The `middleware!` macro
/// supports annotating the macro so as to drive the inference allowing the handler
/// code to remain with minimal annotations.
///
/// ```
/// # #[macro_use] extern crate nickel;
/// # fn main() {
/// # struct MyServerData;
/// middleware! { |_request, _response| <MyServerData>
///     // _response is of type Response<MyServerData>
///     "Hello World"
/// }
/// # ; // This semicolon is required to satisfy returning `()`
/// # }
/// ```
#[macro_export]
macro_rules! middleware {
    (|$req:tt, mut $res:ident| <$data:path> $($b:tt)+) => { _middleware_inner!($req, $res, mut $res, <$data> $($b)+) };
    (|$req:tt, $res:ident| <$data:path> $($b:tt)+) => { _middleware_inner!($req, $res, $res, <$data> $($b)+) };
    (|$req:tt| <$data:path> $($b:tt)+) => { middleware!(|$req, _res| <$data> $($b)+) };
    (|$req:tt, mut $res:ident| $($b:tt)+) => { _middleware_inner!($req, $res, mut $res, $($b)+) };
    (|$req:tt, $res:ident| $($b:tt)+) => { _middleware_inner!($req, $res, $res, $($b)+) };
    (|$req:tt| $($b:tt)+) => { middleware!(|$req, _res| $($b)+) };
    ($($b:tt)+) => { middleware!(|_, _res| $($b)+) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _middleware_inner {
    ($req:tt, $res:ident, $res_binding:pat, <$data:path> $($b:tt)+) => {{
        use $crate::{MiddlewareResult,Responder, Response, Request};

        #[inline(always)]
        fn restrict<'mw, R: Responder<$data>>(r: R, res: Response<'mw, $data>)
                -> MiddlewareResult<'mw, $data> {
            res.send(r)
        }

        // Inference fails due to thinking it's a (&Request, Response) with
        // different mutability requirements
        #[inline(always)]
        fn restrict_closure<F>(f: F) -> F
            where F: for<'r, 'mw>
                        Fn(&'r mut Request<'mw, $data>, Response<'mw, $data>)
                            -> MiddlewareResult<'mw, $data> + Send + Sync { f }

        restrict_closure(move |as_pat!($req), $res_binding| {
            restrict(as_block!({$($b)+}), $res)
        })
    }};
    ($req:tt, $res:ident, $res_binding:pat,  $($b:tt)+) => {{
        use $crate::{MiddlewareResult,Responder, Response, Request};

        #[inline(always)]
        fn restrict<'mw, D, R: Responder<D>>(r: R, res: Response<'mw, D>)
                -> MiddlewareResult<'mw, D> {
            res.send(r)
        }

        // Inference fails due to thinking it's a (&Request, Response) with
        // different mutability requirements
        #[inline(always)]
        fn restrict_closure<F, D>(f: F) -> F
            where F: for<'r, 'mw>
                        Fn(&'r mut Request<'mw, D>, Response<'mw, D>)
                            -> MiddlewareResult<'mw, D> + Send + Sync { f }

        restrict_closure(move |as_pat!($req), $res_binding| {
            restrict(as_block!({$($b)+}), $res)
        })
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! as_block { ($b:block) => ( $b ) }

#[doc(hidden)]
#[macro_export]
macro_rules! as_pat { ($p:pat) => ( $p ) }
