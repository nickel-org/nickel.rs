#[macro_use] extern crate nickel;

use nickel::{Nickel, Request, Response, Middleware, MiddlewareResult};

fn logger_fn<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("logging request from logger fn: {:?}", req.origin.uri);
    res.next_middleware()
}

struct Logger;

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>)
    -> MiddlewareResult<'mw, D> {
        println!("logging request from logger middleware: {:?}", req.origin.uri);
        res.next_middleware()
    }
}

fn main() {
    let mut server = Nickel::new();

    // Middleware is optional and can be registered with `utilize`

    // This is an example middleware function that just logs each request
    // The middleware! macro wraps a closure which can capture variables
    // from the outer scope. See `example_route_data` for an example.
    server.utilize(middleware! { |request|
        println!("logging request from middleware! macro: {:?}", request.origin.uri);
    });

    // Middleware can also be regular rust functions or anything that implements
    // the `Middleware` trait.
    server.utilize(logger_fn);
    server.utilize(Logger);

    server.listen("127.0.0.1:6767");
}
