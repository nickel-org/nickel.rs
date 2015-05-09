#[macro_export]
macro_rules! router {
    ($($input:tt)*) => {{
            use $crate::HttpRouter;
            let mut router = $crate::Router::new();

            _router_inner!(router $($input)*)
    }}
}

#[macro_export]
macro_rules! _router_inner {
    ($router:ident)
        => { $router }; // Base case
    ($router:ident $method:ident $path:expr => |$req:tt, mut $res:ident| { $($b:tt)* } $($rest:tt)*)
        => {{
            $router.$method($path, middleware!(|$req, mut $res| $($b)*));

            _router_inner!($router $($rest)*)
        }};
    ($router:ident $method:ident $path:expr => |$req:tt, $res:ident| { $($b:tt)* } $($rest:tt)*)
        => {{
            $router.$method($path, middleware!(|$req, $res| $($b)*));

            _router_inner!($router $($rest)*)
        }};
    ($router:ident $method:ident $path:expr => |$req:tt| { $($b:tt)* } $($rest:tt)*)
        => {
            _router_inner!($router $method $path => |$req, _res| { $($b)* } $($rest)*)
        };
    ($router:ident $method:ident $path:expr => { $($b:tt)* } $($rest:tt)*)
        => {
            _router_inner!($router $method $path => |_, _res| { $($b)* } $($rest)*)
        };
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
/// server.get("/", middleware! {
///     format!("{}", visits.fetch_add(1, Ordering::Relaxed))
/// });
///
/// server.listen("127.0.0.1:6767");
/// # }
/// ```
#[macro_export]
macro_rules! middleware {
    (|$req:tt, mut $res:ident| $($b:tt)+) => { middleware__inner!($req, $res, mut $res, $($b)+) };
    (|$req:tt, $res:ident| $($b:tt)+) => { middleware__inner!($req, $res, $res, $($b)+) };
    (|$req:tt| $($b:tt)+) => { middleware!(|$req, _res| $($b)+) };
    ($($b:tt)+) => { middleware!(|_, _res| $($b)+) };
}

#[doc(hidden)]
#[macro_export]
macro_rules! middleware__inner {
    ($req:tt, $res:ident, $res_binding:pat, $($b:tt)+) => {{
        use $crate::{MiddlewareResult,ResponseFinalizer, Response, Request};

        #[inline(always)]
        fn restrict<'a, R: ResponseFinalizer>(r: R, res: Response<'a>)
                -> MiddlewareResult<'a> {
            res.send(r)
        }

        // Inference fails due to thinking it's a (&Request, Response) with
        // different mutability requirements
        #[inline(always)]
        fn restrict_closure<F>(f: F) -> F
            where F: for<'r, 'b, 'a>
                        Fn(&'r mut Request<'b, 'a, 'b>, Response<'a>)
                            -> MiddlewareResult<'a> + Send + Sync { f }

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
