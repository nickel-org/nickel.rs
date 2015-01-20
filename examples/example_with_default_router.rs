#![allow(unstable)]

extern crate serialize;
extern crate nickel;
extern crate http;

use nickel::{Nickel, Request, Response, HttpRouter};
use std::io::net::ip::Ipv4Addr;

fn main() {
    let mut server = Nickel::new();

    fn bar_handler (_request: &Request, response: &mut Response) {
        response.send("This is the /bar handler");
    }

    // issue #20178
    let bhandler: fn(&Request, &mut Response) = bar_handler;

    // go to http://localhost:6767/bar to see this route in action
    server.get("/bar", bhandler);

    fn foo_handler (request: &mut Request, _response: &mut Response) -> String {
        format!("Foo is '{}'", request.param("foo"))
    }

    let fhandler: fn(&mut Request, &mut Response) -> String = foo_handler;

    // go to http://localhost:6767/foo to see this route in action
    server.get("/:foo", fhandler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
