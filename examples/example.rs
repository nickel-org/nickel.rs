extern crate serialize;
extern crate nickel;
extern crate http;

use http::status::{NotFound, BadRequest};
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

    fn user_handler(request: &Request, _response: &mut Response) -> String {
        format!("This is user: {}", request.param("userid"))
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

    fn double_wildcard(_request: &Request, _response: &mut Response) -> &'static str {
        "This matches /a/crazy/route and also /a/super/crazy/route"
    }

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", double_wildcard);

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler(request: &Request, _response: &mut Response) -> String {
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    }

    // go to http://localhost:6767/a/post/request to see this route in action
    router.post("/a/post/request", post_handler);

    // try calling http://localhost:6767/query?foo=bar
    fn query_handler(request: &Request, _response: &mut Response) -> String {
        format!("Your foo values in the query string are: {}", request.query("foo", "This is only a default value!"))
    }

    router.get("/query", query_handler);

    // try calling http://localhost:6767/strict?state=valid
    // then try calling http://localhost:6767/strict?state=invalid
    fn strict_handler(request: &Request, response: &mut Response) -> MiddlewareResult {
        if request.query("state", "invalid")[0].as_slice() != "valid" {
            Err(NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest)))
        } else {
            response.send("Congratulations on conforming!");
            Ok(Halt)
        }
    }

    router.get("/strict", strict_handler);

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
