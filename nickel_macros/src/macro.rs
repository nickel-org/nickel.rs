#![feature(macro_rules)]

#[macro_export]
macro_rules! router (
    ($($method:ident $path:expr => |$req:ident, $res:ident| $b:block)+) => (
        {
            use nickel::HttpRouter;
            let mut router = nickel::Router::new();

            $(
                {
                    #[allow(unused_variable)]
                    fn f($req: &nickel::Request, $res: &mut nickel::Response) $b
                    router.$method($path, f)
                }
            );*

            router
        }
    )
)
