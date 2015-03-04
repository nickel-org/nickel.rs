#![feature(net)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;

use nickel::{Nickel, HttpRouter};
use std::net::IpAddr;

fn main() {
    let mut server = Nickel::new();

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", middleware! { "This is the /bar handler" });

    // go to http://localhost:6767/foo to see this route in action
    server.get("/:foo", middleware! { |request|
        format!("Foo is '{}'", request.param("foo"))
    });

    server.listen(IpAddr::new_v4(127, 0, 0, 1), 6767);
}
