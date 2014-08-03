extern crate serialize;
extern crate nickel;

use nickel::{ Nickel, Action, Continue, Request, Response, FromFn };
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
    fn logger (request: &Request, _response: &mut Response) -> Action {
        println!("logging request: {}", request.origin.request_uri);

        // a request is supposed to return a `bool` to indicate whether additional
        // middleware should continue executing or should be stopped.
        Continue
    }

    // middleware is optional and can be registered with `utilize`
    server.utilize(FromFn::new(logger));

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(Nickel::static_files("examples/assets/"));

    fn user_handler (request: &Request, response: &mut Response) {
        let text = format!("This is user: {}", request.params.get(&"userid".to_string()));
        response.send(text.as_slice());
    }

    // go to http://localhost:6767/user/4711 to see this route in action
    server.get("/user/:userid", user_handler);

    fn bar_handler (_request: &Request, response: &mut Response) {
        response.send("This is the /bar handler");
    }

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    fn simple_wildcard (_request: &Request, response: &mut Response) {
        response.send("This matches /some/crazy/route but not /some/super/crazy/route");
    }

    // go to http://localhost:6767/some/crazy/route to see this route in action
    server.get("/some/*/route", simple_wildcard);

    fn double_wildcard (_request: &Request, response: &mut Response) {
        response.send("This matches /a/crazy/route and also /a/super/crazy/route");
    }

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    server.get("/a/**/route", double_wildcard);

    // this will cause json bodies automatically being parsed
    server.utilize(Nickel::json_body_parser());

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler (request: &Request, response: &mut Response) {

        let person = request.json_as::<Person>().unwrap();
        let text = format!("Hello {} {}", person.firstname, person.lastname);
        response.send(text.as_slice());
    }

    // go to http://localhost:6767/a/post/request to see this route in action
    server.post("/a/post/request", post_handler);

    // this will cause the query string to be parsed on each request
    server.utilize(Nickel::query_string());

    // try calling http://localhost:6767/query?foo=bar
    fn query_handler (request: &Request, response: &mut Response) {
        let text = format!("Your foo values in the query string are: {}", request.query("foo", "This is only a default value!"));
        response.send(text.as_slice());
    }

    server.get("/query", query_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
