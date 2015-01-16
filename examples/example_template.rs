#![feature(old_io)]

extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter};
use std::old_io::net::ip::Ipv4Addr;
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    fn root_handler (_request: &Request, response: &mut Response) {
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "user");
        response.render("examples/assets/template.tpl", &data);
    }

    // issue #20178
    let handler: fn(&Request, &mut Response) = root_handler;

    server.get("/", handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
