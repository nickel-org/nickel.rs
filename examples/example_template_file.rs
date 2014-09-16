extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{ Nickel, Handler, Request};
use std::io::net::ip::Ipv4Addr;
use std::collections::HashMap;

fn main() {

    let mut server = Nickel::new();

    fn user_handler(_: &Request, map: &mut HashMap<String, String>) {
        map.insert("name".to_string(), "world".to_string());
    }

    // go to http://localhost:6767/bar to see this route in action
    server.get("/", Handler::new_from_file("examples/assets/test.htm", Some(user_handler)));

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
