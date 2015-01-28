#![allow(unstable)]

extern crate serialize;
extern crate nickel;
extern crate http;

use http::status::{NotFound, BadRequest};
use nickel::{
    Nickel, NickelError, ErrorWithStatusCode, Continue, Halt, Request, Response,
    QueryString, JsonBody, StaticFilesHandler, MiddlewareResult, HttpRouter
};
use nickel::mimes::MediaType;
use std::old_io::net::ip::Ipv4Addr;
use std::collections::BTreeMap;
use serialize::json::{Json, ToJson};

#[derive(Decodable, Encodable)]
struct Person {
    firstname: String,
    lastname:  String,
}

impl ToJson for Person {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("first_name".to_string(), self.firstname.to_json());
        map.insert("last_name".to_string(), self.lastname.to_json());
        Json::Object(map)
    }
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
    // issue #20178
    let logger_handler: fn(&Request, &mut Response) -> MiddlewareResult = logger;
    server.utilize(logger_handler);

    let mut router = Nickel::router();

    fn user_handler(request: &Request, _response: &mut Response) -> String {
        format!("This is user: {}", request.param("userid"))
    }

    // issue #20178
    let uhandler: fn(&Request, &mut Response) -> String = user_handler;

    // go to http://localhost:6767/user/4711 to see this route in action
    router.get("/user/:userid", uhandler);

    fn bar_handler(_request: &Request, response: &mut Response) {
        response.send("This is the /bar handler");
    }

    // issue #20178
    let bhandler: fn(&Request, &mut Response) = bar_handler;

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", bhandler);

    fn simple_wildcard(_request: &Request, response: &mut Response) {
        response.send("This matches /some/crazy/route but not /some/super/crazy/route");
    }

    // issue #20178
    let shandler: fn(&Request, &mut Response) = simple_wildcard;

    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", shandler);

    fn double_wildcard(_request: &Request, _response: &mut Response) -> &'static str {
        "This matches /a/crazy/route and also /a/super/crazy/route"
    }

    // issue #20178
    let dhandler: fn(&Request, &mut Response) -> &'static str = double_wildcard;

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", dhandler);

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    fn post_handler(request: &mut Request, _response: &mut Response) -> String {
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    }

    // issue #20178
    let phandler: fn(&mut Request, &mut Response) -> String = post_handler;

    // go to http://localhost:6767/a/post/request to see this route in action
    router.post("/a/post/request", phandler);

    fn json_response(_request: &Request, _response: &mut Response) -> Json {
        let person = Person {
            firstname: "Pea".to_string(),
            lastname: "Nut".to_string()
        };
        person.to_json()
    }

    // issue #20178
    let jresponse: fn(&Request, &mut Response) -> Json = json_response;

    // go to http://localhost:6767/api/person/1 to see this route in action
    router.get("/api/person/1", jresponse);

    // try calling http://localhost:6767/query?foo=bar
    fn query_handler(request: &mut Request, _response: &mut Response) -> String {
        format!("Your foo values in the query string are: {:?}", *request.query("foo", "This is only a default value!"))
    }

    // issue #20178
    let qhandler: fn(&mut Request, &mut Response) -> String = query_handler;

    router.get("/query", qhandler);

    // try calling http://localhost:6767/strict?state=valid
    // then try calling http://localhost:6767/strict?state=invalid
    fn strict_handler(request: &mut Request, response: &mut Response) -> MiddlewareResult {
        if request.query("state", "invalid")[0].as_slice() != "valid" {
            Err(NickelError::new("Error Parsing JSON", ErrorWithStatusCode(BadRequest)))
        } else {
            response.send("Congratulations on conforming!");
            Ok(Halt)
        }
    }

    // issue #20178
    let sthandler: fn(&mut Request, &mut Response) -> MiddlewareResult = strict_handler;

    router.get("/strict", sthandler);

    server.utilize(router);

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404(err: &NickelError, _req: &Request, response: &mut Response) -> MiddlewareResult {
        match err.kind {
            ErrorWithStatusCode(NotFound) => {
                response.content_type(MediaType::Html)
                        .status_code(NotFound)
                        .send("<h1>Call the police!<h1>");
                Ok(Halt)
            },
            _ => Ok(Continue)
        }
    }

    // issue #20178
    let custom_handler: fn(&NickelError, &Request, &mut Response) -> MiddlewareResult = custom_404;

    server.handle_error(custom_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
