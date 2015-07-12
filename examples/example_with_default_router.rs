#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", middleware! { "This is the /bar handler" });

    // go to http://localhost:6767/foo to see this route in action
    server.get("/:foo", middleware! { |request|
        format!("Foo is '{}'", request.param("foo").unwrap())
    });

    server.listen("127.0.0.1:6767");
}
