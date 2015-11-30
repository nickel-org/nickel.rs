#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, MiddlewareResult, HttpRouter};

fn logger<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger fn: {:?}", req.origin.uri);
    res.next_middleware()
}

fn main() {
    let mut server = Nickel::new();

    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    // Middleware is optional and can be registered with `utilize`

    // This is an example middleware function that just logs each request
    // The middleware! macro wraps a closure which can capture variables
    // from the outer scope. See `example_route_data` for an example.
    server.utilize(middleware! { |request|
        println!("logging request from middleware! macro: {:?}", request.origin.uri);
    });

    // Middleware can also be regular rust functions
    server.utilize(logger);

    let mut router = Nickel::router();

    // go to http://localhost:6767/user/4711 to see this route in action
    router.get("/user/:userid", middleware! { |request|
        format!("This is user: {}", request.param("userid").unwrap())
    });

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", middleware!("This is the /bar handler"));

    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", middleware! {
        "This matches /some/crazy/route but not /some/super/crazy/route"
    });

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", middleware! {
        "This matches /a/crazy/route and also /a/super/crazy/route"
    });

    server.utilize(router);

    server.listen("127.0.0.1:6767");
}
