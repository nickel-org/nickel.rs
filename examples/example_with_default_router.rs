extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{ Nickel, Request, Response };
use std::io::net::ip::Ipv4Addr;

fn main() {

    let mut server = Nickel::new();

    fn bar_handler (_request: &Request, response: &mut Response) {
        response.send("This is the /bar handler");
    }

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bar_handler);

    fn foo_handler (_request: &Request, response: &mut Response) {
        response.send("This is the /foo handler");
    }

    // go to http://localhost:6767/foo to see this route in action
    server.get("/foo", foo_handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
