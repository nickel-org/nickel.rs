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

#[macro_export]
macro_rules! middleware {
    (@$f:ident) => {{
        // issue #20178 with a lame interaction from #23630. *grumble*
        let f : for<'a> fn(&mut Request, Response<'a>) -> MiddlewareResult<'a> = $f;
        f
    }};
    (|$req:ident, $res:ident| $($b:tt)+) => {{
        use $crate::{MiddlewareResult,ResponseFinalizer, Response, Request};

        #[inline(always)]
        fn restrict<'a, R: ResponseFinalizer>(r: R, res: Response<'a>)
                -> MiddlewareResult<'a> {
            r.respond(res)
        }

        #[inline(always)]
        fn ignore_unused(_: &Request, _: &Response) {}

        let f: Box<for<'r, 'b, 'a> Fn(&'r mut Request<'b, 'a, 'b>, Response<'a>)
                                        -> MiddlewareResult<'a> + Send + Sync>
                    = Box::new(move |$req, $res| {
                          ignore_unused($req, &$res);
                          restrict(as_block!({$($b)+}), $res)
                      });

        f

    }};
    (|$req:ident| $($b:tt)+) => { middleware!(|$req, res| $($b)+) };
    ($($b:tt)+) => { middleware!(|req, res| $($b)+) };
}

#[macro_export] macro_rules! as_block { ($b:block) => ( $b ) }
