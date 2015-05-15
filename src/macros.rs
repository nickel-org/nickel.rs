#[macro_export]
macro_rules! router {
    ($($method:ident $path:expr => |$req:ident, $res:ident| $b:block)+) => (
        {
            use $crate::HttpRouter;
            let mut router = $crate::Router::new();

            $( router.$method($path, middleware!(|$req, $res| $b)); )+

            router
        }
    )
}

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
/// server.get("/", middleware! { |_req, _res|
///     format!("{}", visits.fetch_add(1, Ordering::Relaxed))
/// });
///
/// server.listen("127.0.0.1:6767");
/// # }
/// ```
#[macro_export]
macro_rules! middleware {
    (|$req:ident, mut $res:ident| $($b:tt)+) => { middleware__inner!($req, $res, mut $res, $($b)+) };
    (|$req:ident, $res:ident| $($b:tt)+) => { middleware__inner!($req, $res, $res, $($b)+) };
    (|$req:ident| $($b:tt)+) => { middleware!(|$req, res| $($b)+) };
    ($($b:tt)+) => { middleware!(|req, res| $($b)+) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! middleware__inner {
    ($req:ident, $res:ident, $res_binding:pat, $($b:tt)+) => {{
        use $crate::{MiddlewareResult,ResponseFinalizer, Response, Request};

        #[inline(always)]
        fn restrict<'a, R: ResponseFinalizer>(r: R, res: Response<'a>)
                -> MiddlewareResult<'a> {
            r.respond(res)
        }

        #[inline(always)]
        fn ignore_unused(_: &Request, _: &Response) {}

        // Inference fails due to thinking it's a (&Request, Response) with
        // different mutability requirements
        #[inline(always)]
        fn restrict_closure<F>(f: F) -> F
            where F: for<'r, 'b, 'a>
                        Fn(&'r mut Request<'b, 'a, 'b>, Response<'a>)
                            -> MiddlewareResult<'a> + Send + Sync { f }

        restrict_closure(move |$req, $res_binding| {
            ignore_unused($req, &$res);
            restrict(as_block!({$($b)+}), $res)
        })
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! as_block { ($b:block) => ( $b ) }
