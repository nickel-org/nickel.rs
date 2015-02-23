#![feature(old_io)]

extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, Halt};
use std::old_io::net::ip::Ipv4Addr;
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    fn root_handler<'a>(_request: &mut Request, response: Response<'a, 'a>)
                        -> MiddlewareResult<'a, 'a> {
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "user");
        Ok(Halt(try!(response.render("examples/assets/template.tpl", &data))))
    }

    // issue #20178
    let handler: for<'a> fn(&mut Request, Response<'a, 'a>)
                    -> MiddlewareResult<'a, 'a> = root_handler;

    server.get("/", handler);

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
