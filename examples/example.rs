extern crate serialize;
extern crate nickel;
extern crate http;

use std::collections::HashMap;
use http::status::NotFound;
use nickel::{
    Nickel, NickelError, ErrorWithStatusCode,
    Action, Continue, Halt, Request,
    Response, IntoMiddleware, IntoErrorHandler,
    Handler, JsonBody, QueryString
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
    fn logger (request: &Request, _response: &mut Response) -> Result<Action, NickelError> {
        println!("logging request: {}", request.origin.request_uri);

        // a request is supposed to return a `bool` to indicate whether additional
        // middleware should continue executing or should be stopped.
        Ok(Continue)
    }

    // middleware is optional and can be registered with `utilize`
    server.utilize(IntoMiddleware::from_fn(logger));

    // this will cause json bodies automatically being parsed
    server.utilize(Nickel::json_body_parser());

    // this will cause the query string to be parsed on each request
    server.utilize(Nickel::query_string());

    let mut router = Nickel::router();

    fn user_handler(request: &Request, map: &mut HashMap<String, String>) {
        map.insert("user_id".to_string(), request.params["userid".to_string()].clone());
    }

    // go to http://localhost:6767/user/4711 to see this route in action
    router.get("/user/:userid", Handler::new("This is user: {{ user_id }}", Some(user_handler)));

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", Handler::new("This is the /bar handler.", None));

    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", Handler::new("This matches /some/crazy/route but not /some/super/crazy/route", None));

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", Handler::new("This matches /a/crazy/route and also /a/super/crazy/route", None));

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler (request: &Request, map: &mut HashMap<String, String>) {
        let person = request.json_as::<Person>().unwrap();
        map.insert("first_name".to_string(), person.firstname);
        map.insert("last_name".to_string(), person.lastname);
    }

    // go to http://localhost:6767/a/post/request to see this route in action
    router.post("/a/post/request", Handler::new("Hello {{ first_name }} {{ last_name }}", Some(post_handler)));

    // try calling http://localhost:6767/query?foo=bar
    fn query_handler (request: &Request, map: &mut HashMap<String, String>) {
        map.insert("foo".to_string(), request.query("foo", "This is only a default value!")
                                .iter()
                                .map(|s| s.as_slice())
                                .collect::<Vec<&str>>()
                                .connect(", ")
                  );
    }

    router.get("/query", Handler::new("Your foo values in the query string are: {{ foo }}", Some(query_handler)));

    server.utilize(router);

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(Nickel::static_files("examples/assets/"));

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404 (err: &NickelError, _req: &Request, response: &mut Response) -> Result<Action, NickelError> {
        match err.kind {
            ErrorWithStatusCode(NotFound) => {
                response.set_content_type("html");
                response.origin.status = NotFound;
                response.send("<h1>Call the police!<h1>");
                Ok(Halt)
            },
            _ => Ok(Continue)
        }
    }

    server.handle_error(IntoErrorHandler::from_fn(custom_404));

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
