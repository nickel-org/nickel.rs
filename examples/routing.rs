#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel, HttpRouter};
use hyper::method::Method;

fn main() {
    let mut server = Nickel::new();

    // Nickel provides a default router on the server for getting
    // up and running quickly. If you want to partition out your app
    // you might want to use an explicit router though.
    server.utilize(explicit_router());

    // Most common HTTP verbs are extension methods added from the HttpRouter trait.
    // You can see other examples such as 'json' to see other verbs in use.
    // For other HTTP verbs, you can use the `add_route` method.

    // go to http://localhost:6767/bar to see this route in action
    server.add_route(Method::Get, "/bar", middleware! {
        "This is the /bar handler"
    });

    // go to http://localhost:6767/foo to see this route in action
    server.get("/:foo", middleware! { |request|
        let foo = request.param("foo").unwrap();
        let format = request.param("format").unwrap();
        format!("Foo is '{}'. The requested format is '{}'", foo, format)
    });

    server.listen("127.0.0.1:6767");
}

fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    // Wildcard '*' routes are supported
    // - '*' will match a single directories seperated by '/'
    // - '**' will match potentially many directories

    // Single wildcard:
    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", middleware! {
        "This matches /some/crazy/route but not /some/super/crazy/route"
    });

    // Double wildcards:
    // go to http://localhost:6767/a/nice/route
    // or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", middleware! {
        "This matches /a/crazy/route and also /a/super/crazy/route"
    });

    router
}
