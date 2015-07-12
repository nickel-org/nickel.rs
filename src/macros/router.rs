#[macro_export]
macro_rules! router {
    ($($input:tt)*) => {{
            use $crate::HttpRouter;
            let mut router = $crate::Router::new();

            _router_inner!(router $($input)*)
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! _router_inner {
    ($router:ident)
        => { $router }; // Base case
    ($router:ident $method:ident $path:expr => |mut $res:ident| { $($b:tt)* } $($rest:tt)*)
        => {{
            $router.$method($path, middleware!(|mut $res| $($b)*));

            _router_inner!($router $($rest)*)
        }};
    ($router:ident $method:ident $path:expr => |$res:ident| { $($b:tt)* } $($rest:tt)*)
        => {{
            $router.$method($path, middleware!(|$res| $($b)*));

            _router_inner!($router $($rest)*)
        }};
    ($router:ident $method:ident $path:expr => { $($b:tt)* } $($rest:tt)*)
        => {
            _router_inner!($router $method $path => |_res| { $($b)* } $($rest)*)
        };
}
