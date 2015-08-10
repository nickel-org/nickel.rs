// If this test ever starts passing (by failing) then we should start looking into
// unboxed closures for handlers again.

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.utilize(|res| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'a, 'k> <[closure@tests/compile-fail
    //~^^ ERROR the type of this value must be known in this context

    server.get("**", |res| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'a, 'k> <[closure@tests/compile-fail
    //~^^ ERROR the type of this value must be known in this context

    server.listen("127.0.0.1:6767");
}
