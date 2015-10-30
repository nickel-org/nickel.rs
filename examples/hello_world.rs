#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn hello_world<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello World")
}

fn main() {
    let mut server = Nickel::new();
    server.get("**", hello_world);
    server.listen("127.0.0.1:6767");
}
