#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Set appropriate headers
    res.set(AccessControlAllowOrigin::Any);
    res.set(AccessControlAllowHeaders(vec![
        // Hyper uses the `unicase::Unicase` type to ensure comparisons are done
        // case-insensitively. Here, we use `into()` to convert to one from a `&str`
        // so that we don't have to import the type ourselves.
        "Origin".into(),
        "X-Requested-With".into(),
        "Content-Type".into(),
        "Accept".into(),
    ]));

    // Pass control to the next middleware
    res.next_middleware()
}

fn main() {
    let mut server = Nickel::new();
    server.utilize(enable_cors);
    server.options("**", middleware!(""));
    server.get("**", middleware!("Hello CORS Enabled World"));
    server.listen("127.0.0.1:6767").unwrap();
}
