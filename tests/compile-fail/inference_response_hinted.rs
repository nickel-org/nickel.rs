// If this test ever starts passing (by failing) then we should start looking into
// unboxed closures for handlers again.

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response};

fn main() {
    let mut server = Nickel::new();

    server.utilize(|_, res: Response| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'r,'b,'a> <[closure tests/compile-fail
    //~^^ ERROR type mismatch: the type `[closure tests/compile-fail

    server.get("**", |_, res: Response| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'r,'b,'a> <[closure tests/compile-fail
    //~^^ ERROR type mismatch: the type `[closure tests/compile-fail

    server.listen("127.0.0.1:6767");
}
