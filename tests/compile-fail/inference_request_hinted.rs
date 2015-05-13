// If this test ever starts passing (by failing) then we should start looking into
// unboxed closures for handlers again.

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response};

fn main() {
    let mut server = Nickel::new();

    // Request hinted
    server.utilize(|_: &mut Request, res| res.send("Hello World!"));
    //~^ ERROR the type of this value must be known in this context

    server.get("**", |_: &mut Request, res| res.send("Hello World!"));
    //~^ ERROR the type of this value must be known in this context

    server.listen("127.0.0.1:6767");
}
