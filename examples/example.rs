#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate regex;

use std::collections::BTreeMap;
use std::io::Write;
use nickel::status::StatusCode::{self, NotFound, BadRequest};
use nickel::{
    Nickel, NickelError, Continue, Halt, Request,
    QueryString, JsonBody, StaticFilesHandler, HttpRouter, Action, MediaType
};
use regex::Regex;
use rustc_serialize::json::{Json, ToJson};

#[derive(RustcDecodable, RustcEncodable)]
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
    // middleware is optional and can be registered with `utilize`
    server.utilize(middleware! { |request|
        println!("logging request: {:?}", request.origin.uri);
    });

    let mut router = Nickel::router();

    // go to http://localhost:6767/user/4711 to see this route in action
    router.get("/user/:userid", middleware! { |request|
        format!("This is user: {}", request.param("userid").unwrap())
    });

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", middleware!("This is the /bar handler"));

    // go to http://localhost:6767/content-type to see this route in action
    router.get("/content-type", middleware! { |_, mut response|
        response.set(MediaType::Json);
        "{'foo':'bar'}"
    });

    let hello_regex = Regex::new("/hello/(?P<name>[a-zA-Z]+)").unwrap();

    // go to http://localhost:6767/hello/moomah to see this route in action
    router.get(hello_regex, middleware! { |request|
        format!("Hello {}", request.param("name").unwrap())
    });

    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get("/some/*/route", middleware! {
        "This matches /some/crazy/route but not /some/super/crazy/route"
    });

    // go to http://localhost:6767/a/nice/route or http://localhost:6767/a/super/nice/route to see this route in action
    router.get("/a/**/route", middleware! {
        "This matches /a/crazy/route and also /a/super/crazy/route"
    });

    // try it with curl
    // curl 'http://localhost:6767/a/post/request' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{ "firstname": "John","lastname": "Connor" }'
    router.post("/a/post/request", middleware! { |request, response|
        let person = request.json_as::<Person>().unwrap();
        format!("Hello {} {}", person.firstname, person.lastname)
    });

    // go to http://localhost:6767/api/person/1 to see this route in action
    router.get("/api/person/1", middleware! {
        let person = Person {
            firstname: "Pea".to_string(),
            lastname: "Nut".to_string()
        };
        person.to_json()
    });

    // try calling http://localhost:6767/query?foo=bar
    router.get("/query", middleware! { |request|
        if let Some(vals) = request.query().all("foo") {
            format!("Your foo values in the query string are: {:?}", vals)
        } else {
            format!("You didn't provide any foo values!")
        }
    });

    // try calling http://localhost:6767/strict?state=valid
    // then try calling http://localhost:6767/strict?state=invalid
    router.get("/strict", middleware! { |request|
        if request.query().get("state") != Some("valid") {
            (BadRequest, "Error Parsing JSON")
        } else {
            (StatusCode::Ok, "Congratulations on conforming!")
        }
    });

    server.utilize(router);

    // go to http://localhost:6767/thoughtram_logo_brain.png to see static file serving in action
    server.utilize(StaticFilesHandler::new("examples/assets/"));

    //this is how to overwrite the default error handler to handle 404 cases with a custom view
    fn custom_404<'a, D>(err: &mut NickelError<D>, _req: &mut Request<D>) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == NotFound {
                let _ = res.write_all(b"<h1>Call the police!</h1>");
                return Halt(())
            }
        }

        Continue(())
    }


    // issue #20178
    let custom_handler: fn(&mut NickelError<()>, &mut Request<()>) -> Action = custom_404;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767");
}
