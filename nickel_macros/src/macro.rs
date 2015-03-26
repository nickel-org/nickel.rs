
#[macro_export]
macro_rules! router {
    ($($method:ident $path:expr => |$req:ident, $res:ident| $b:block)+) => (
        {
            use nickel::HttpRouter;
            let mut router = nickel::Router::new();

            $( router.$method($path, middleware!(|$req, $res| $b)); )+

            router
        }
    )
}

#[macro_export]
macro_rules! middleware {
    (@$f:ident) => {{
        // issue #20178 with a lame interaction from #23630. *grumble*
        let f : for<'a> fn(&mut Request, Response<'a>) -> MiddlewareResult<'a> = $f;
        f
    }};
    (|$req:ident, $res:ident| $($b:tt)+) => {{
        use nickel::{MiddlewareResult,ResponseFinalizer, Response, Request};

        #[inline(always)]
        fn restrict<'a, R: ResponseFinalizer>(r: R, res: Response<'a>)
                -> MiddlewareResult<'a> {
            r.respond(res)
        }

        #[allow(unused_variables)]
        fn f<'a>($req: &mut Request, $res: Response<'a>)
                -> MiddlewareResult<'a> {
            restrict(as_block!({$($b)+}), $res)
        }

        middleware!(@f)
    }};
    (|$req:ident| $($b:tt)+) => { middleware!(|$req, res| $($b)+) };
    ($($b:tt)+) => { middleware!(|req, res| $($b)+) };
}

#[macro_export] macro_rules! as_block { ($b:block) => ( $b ) }
