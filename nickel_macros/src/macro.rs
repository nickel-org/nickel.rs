
#[macro_export]
macro_rules! router {
    ($($method:ident $path:expr => |$req:ident, $res:ident| $b:block)+) => (
        {
            use nickel::{HttpRouter, MiddlewareResult};
            use nickel::ResponseFinalizer;
            let mut router = nickel::Router::new();

            #[inline(always)]
            fn restrict<'a, R: ResponseFinalizer>(r: R, res: nickel::Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
                r.respond(res)
            }

            $(
                {
                    #[allow(unused_variables)]
                    fn f<'a>($req: &mut nickel::Request, $res: nickel::Response<'a, 'a>) -> MiddlewareResult<'a, 'a> {
                        restrict($b, $res)
                    }

                    // issue #20178
                    let fhandler: for<'a> fn(&mut nickel::Request, nickel::Response<'a, 'a>) -> MiddlewareResult<'a, 'a> = f;

                    router.$method($path, fhandler);
                }
            )+

            router
        }
    )
}
