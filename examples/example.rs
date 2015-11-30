#[macro_use] extern crate nickel;

use std::io::Write;
use nickel::status::StatusCode::NotFound;
use nickel::{
    Nickel, NickelError, Continue, Halt, Request, Response, MiddlewareResult,
    StaticFilesHandler, HttpRouter, Action
};

fn logger<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger fn: {:?}", req.origin.uri);
    Ok(Continue(res))
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

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404<'a>(err: &mut NickelError, _req: &mut Request) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                let _ = res.write_all(b"<h1>Call the police!</h1>");
                return Halt(())
            }
        }

        Continue(())
    }


    // issue #20178
    let custom_handler: fn(&mut NickelError, &mut Request) -> Action = custom_404;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767");
}
