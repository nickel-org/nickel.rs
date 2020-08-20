#[macro_use] extern crate nickel;

use async_trait::async_trait;
use nickel::{Nickel, Request, Response, Middleware, MiddlewareResult};

fn logger_fn(req: &mut Request, res: Response) -> MiddlewareResult {
    println!("logging request from logger fn: {:?}", req.origin.uri());
    res.next_middleware()
}

struct Logger;

#[async_trait]
impl<D: Send + 'static + Sync> Middleware<D> for Logger {
    async fn invoke(&self, req: &mut Request<D>, res: Response<D>)
    -> MiddlewareResult<D> {
        println!("logging request from logger middleware: {:?}", req.origin.uri());
        res.next_middleware()
    }
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    // Middleware is optional and can be registered with `utilize`

    // This is an example middleware function that just logs each request
    // The middleware! macro wraps a closure which can capture variables
    // from the outer scope. See `example_route_data` for an example.
    server.utilize(middleware! { |request|
        println!("logging request from middleware! macro: {:?}", request.origin.uri());
    });

    // Middleware can also be regular rust functions or anything that implements
    // the `Middleware` trait.
    server.utilize(logger_fn);
    server.utilize(Logger);

    server.listen("127.0.0.1:6767").await.unwrap();
}
