extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{ Nickel, Handler};
use std::io::net::ip::Ipv4Addr;

fn main() {

    let mut server = Nickel::new();

    let bar_handler = Handler::new("This is the /bar handler", None);

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    let foo_handler = Handler::new("This is the /foo handler", None);

    // go to http://localhost:6767/foo to see this route in action
    server.get("/foo", foo_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
