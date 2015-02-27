#![feature(old_io)]
extern crate nickel;
#[macro_use] extern crate nickel_macros;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, Halt};
use std::old_io::net::ip::Ipv4Addr;
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    fn handler<'a>(_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "user");
        Ok(Halt(try!(res.render("examples/assets/template.tpl", &data))))
    }

    server.get("/", middleware!(@handler));

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
