nickel.rs
=======

nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
* Variables in routes. Just write `my/route/:someid`
* Easy parameter access: `request.param("someid")`
* simple wildcard routes: `/some/*/route`
* double wildcard routes: `/a/**/route`
* middleware
    * static file support


### Status
[![Build Status](https://travis-ci.org/nickel-org/nickel.rs.png?branch=master)](https://travis-ci.org/nickel-org/nickel.rs)

##[Jump to the nickel.rs website](http://nickel.rs)

#Getting started
The easiest way to get started is to get the example running and play around with it. Let's do that real quick!

##Clone the repository

```shell
git clone --recursive https://github.com/nickel-org/nickel.git
```

##Build nickel

```shell
make all
```

##Run the example

```shell
make run
```

Then try `localhost:6767/user/4711` and `localhost:6767/bar`


##Take a look at the example code
Here is how sample server in `example.rs` looks like:

```rust
extern crate serialize;
extern crate nickel;
extern crate http;

use http::status::NotFound;
use nickel::{
    Nickel, NickelError, ErrorWithStatusCode, Continue, Halt, Request, Response,
    QueryString, JsonBody, StaticFilesHandler, MiddlewareResult, HttpRouter
};
use std::io::net::ip::Ipv4Addr;

#[deriving(Decodable, Encodable)]
struct Person {
    firstname: String,
    lastname:  String,
}

fn main() {
    let mut server = Nickel::new();

    // we would love to use a closure for the handler but it seems to be hard
    // to achieve with the current version of rust.

    //this is an example middleware function that just logs each request
    fn logger(request: &Request, _response: &mut Response) -> MiddlewareResult {
        println!("logging request: {}", request.origin.request_uri);

        // a request is supposed to return a `bool` to indicate whether additional
        // middleware should continue executing or should be stopped.
        Ok(Continue)
    }

    // middleware is optional and can be registered with `utilize`
    server.utilize(logger);

    // this will cause json bodies automatically being parsed
    server.utilize(Nickel::json_body_parser());

    // this will cause the query string to be parsed on each request
    server.utilize(Nickel::query_string());

    let mut router = Nickel::router();

    fn user_handler(request: &Request, response: &mut Response) {
        let text = format!("This is user: {}", request.param("userid"));
        response.send(text.as_slice());
    }

    // go to http://localhost:6767/user/4711 to see this route in action
    router.get("/user/:userid", user_handler);

    fn bar_handler(_request: &Request, response: &mut Response) {
        response.send("This is the /bar handler");
    }

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", bar_handler);

    fn simple_wildcard(_request: &Request, response: &mut Response) {
        response.send("This matches /some/crazy/route but not /some/super/crazy/route");
    }

    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", simple_wildcard);

    fn double_wildcard(_request: &Request, response: &mut Response) {
        response.send("This matches /a/crazy/route and also /a/super/crazy/route");
    }

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", double_wildcard);

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler(request: &Request, response: &mut Response) {

        let person = request.json_as::<Person>().unwrap();
        let text = format!("Hello {} {}", person.firstname, person.lastname);
        response.send(text.as_slice());
    }

    // go to http://localhost:6767/a/post/request to see this route in action
    router.post("/a/post/request", post_handler);

    // try calling http://localhost:6767/query?foo=bar
    fn query_handler(request: &Request, response: &mut Response) {
        let text = format!("Your foo values in the query string are: {}", request.query("foo", "This is only a default value!"));
        response.send(text.as_slice());
    }

    router.get("/query", query_handler);

    server.utilize(router);

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404(err: &NickelError, _req: &Request, response: &mut Response) -> MiddlewareResult {
        match err.kind {
            ErrorWithStatusCode(NotFound) => {
                response.content_type("html")
                        .status_code(NotFound)
                        .send("<h1>Call the police!<h1>");
                Ok(Halt)
            },
            _ => Ok(Continue)
        }
    }

    server.handle_error(custom_404);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
```

##[Jump to the Full Documentation](http://nickel-org.github.io/nickel/)

##License

Nickel is open source and licensed with the [MIT license](https://github.com/nickel-org/nickel/blob/master/LICENSE)


##Contributing

Nickel.rs is a community effort. We welcome new contributors with open arms. 
There is list of [open issues](https://github.com/nickel-org/nickel/issues?state=open) right here on github.

If you need a helping hand reach out to [@cburgdorf](https://github.com/cburgdorf), [@Ryman](https://github.com/Ryman) or [@SimonPersson](https://github.com/SimonPersson).

Make sure to follow this [commit message convention](https://github.com/ajoslin/conventional-changelog/blob/master/CONVENTIONS.md) because we will auto generate a changelog with [clog](https://github.com/thoughtram/clog) in the future.

And hey, did you know you can also contribute by just starring the project here on github :)
