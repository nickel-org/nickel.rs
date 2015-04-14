nickel.rs
=======

[![Join the chat at https://gitter.im/nickel-org/nickel.rs](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/nickel-org/nickel.rs?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. It's API is inspired by the popular express framework for JavaScript.

Some of the features are:

* Easy handlers: A handler is just a function that takes a `&Request` and `&mut Response`
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
cargo build --release
```

##Run the tests

```shell
cargo test
```

##Run the example

```shell
cargo run --example example
```

Then try `localhost:6767/user/4711` and `localhost:6767/bar`


##Take a look at the example code
Here is how sample server in `example.rs` looks like:

```rust
extern crate rustc_serialize;
extern crate nickel;
extern crate regex;
#[macro_use] extern crate nickel_macros;

use std::collections::BTreeMap;
use std::io::Write;
use nickel::status::StatusCode::{self, NotFound, BadRequest};
use nickel::{
    Nickel, NickelError, Continue, Halt, Request,
    QueryString, JsonBody, StaticFilesHandler, HttpRouter, Action
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
        format!("This is user: {}", request.param("userid"))
    });

    // go to http://localhost:6767/bar to see this route in action
    router.get("/bar", middleware!("This is the /bar handler"));

    let hello_regex = Regex::new("/hello/(?P<name>[a-zA-Z]+)").unwrap();

    // go to http://localhost:6767/hello/moomah to see this route in action
    router.get(hello_regex, middleware! { |request|
        format!("Hello {}", request.param("name"))
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
        format!("Your foo values in the query string are: {:?}",
                request.query("foo", "This is only a default value!"))
    });

    // try calling http://localhost:6767/strict?state=valid
    // then try calling http://localhost:6767/strict?state=invalid
    router.get("/strict", middleware! { |request|
        if &*request.query("state", "invalid")[0] != "valid" {
            (BadRequest, "Error Parsing JSON")
        } else {
            (StatusCode::Ok, "Congratulations on conforming!")
        }
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
