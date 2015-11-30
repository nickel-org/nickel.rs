#[macro_use] extern crate nickel;
extern crate regex;

use nickel::{Nickel, HttpRouter};
use regex::Regex;

fn main() {
    let mut server = Nickel::new();

    let hello_regex = Regex::new("/hello/(?P<name>[a-zA-Z]+)").unwrap();
    // go to http://localhost:6767/hello/moomah to see this route in action
    server.get(hello_regex, middleware! { |request|
        format!("Hello {}", request.param("name").unwrap())
    });

    server.listen("127.0.0.1:6767");
}
