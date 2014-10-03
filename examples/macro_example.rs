#![feature(phase)]


extern crate http;
extern crate nickel;
extern crate serialize;
#[phase(plugin)] extern crate nickel_macros;

use http::status;
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

//this is an example middleware function that just logs each request
fn logger(request: &Request, _response: &mut Response) -> MiddlewareResult {
    println!("logging request: {}", request.origin.request_uri);

    // a request is supposed to return a `bool` to indicate whether additional
    // middleware should continue executing or should be stopped.
    Ok(Continue)
}

//this is how to overwrite the default error handler to handle 404 cases with a custom view
fn custom_404(err: &NickelError, _req: &Request, response: &mut Response) -> MiddlewareResult {
    match err.kind {
        ErrorWithStatusCode(status::NotFound) => {
            response.content_type("html")
                    .status_code(status::NotFound)
                    .send("<h1>Call the police!<h1>");
            Ok(Halt)
        },
        _ => Ok(Continue)
    }
}

fn main() {
    let mut server = Nickel::new();

    // middleware is optional and can be registered with `utilize`
    server.utilize(logger);

    // this will cause json bodies automatically being parsed
    server.utilize(Nickel::json_body_parser());

    // this will cause the query string to be parsed on each request
    server.utilize(Nickel::query_string());

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    server.utilize(router! {
        // go to http://localhost:6767/user/4711 to see this route in action
        get "/user/:userid" => |request, response| {
            format!("This is user: {}", request.param("userid"))
        }

        // go to http://localhost:6767/no_alloc/4711 to see this route in action
        get "/no_alloc/:userid" => |request, response| {
            ["This is user: ", request.param("userid")]
        }

        // go to http://localhost:6767/bar to see this route in action
        get "/bar" => |request, response| {
            (200, "This is the /bar handler")
        }

        // go to http://localhost:6767/private to see this route in action
        get "/private" => |request, response| {
            (status::Unauthorized, "This is a private place")
        }

        // go to http://localhost:6767/some/crazy/route to see this route in action
        get "/some/*/route" => |request, response| {
            "This matches /some/crazy/route but not /some/super/crazy/route"
        }

        // go to http://localhost:6767/some/crazy/route to see this route in action
        get "/a/**/route" => |request, response| {
            "This matches /a/crazy/route and also /a/super/crazy/route"
        }

        // try it with curl
        // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
        post "/a/post/request" => |request, response| {
            let person = request.json_as::<Person>().unwrap();
            let text = format!("Hello {} {}", person.firstname, person.lastname);
            response.send(text.as_slice());
        }

        // try calling http://localhost:6767/query?foo=bar
        get "/query" => |request, response| {
            let text = format!("Your foo values in the query string are: {}", request.query("foo", "This is only a default value!"));
            response.send(text.as_slice());
        }
    });

    server.handle_error(custom_404);

    println!("Running server!");
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
