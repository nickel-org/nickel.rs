// If this test ever starts passing (by failing) then we should start looking into
// unboxed closures for handlers again.

#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response};

fn main() {
    let mut server = Nickel::new();

    server.utilize(|_: &mut Request, res: Response| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'r,'b,'a> <[closure tests/com

    server.get("**", |_: &mut Request, res: Response| res.send("Hello World!"));
    //~^ ERROR type mismatch resolving `for<'r,'b,'a> <[closure tests/com

    server.listen("127.0.0.1:6767");
}
