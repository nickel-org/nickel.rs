// If this test ever starts passing (by failing) then we should start looking into
// unboxed closures for handlers again.

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response};

fn main() {
    let mut server = Nickel::new();

    server.utilize(|_, res| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'
    //~^^ ERROR the type of this value must be known in this context
    //~^^^ ERROR type mismatch: the type `[closure@tests/compile

    server.get("**", |_, res| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'
    //~^^ ERROR the type of this value must be known in this context
    //~^^^ ERROR type mismatch: the type `[closure@tests/compile

    server.listen("127.0.0.1:6767");
}
