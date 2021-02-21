#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use hyper::header::{self, HeaderValue};

fn enable_cors(_req: &mut Request, mut res: Response) -> MiddlewareResult {
    // Set appropriate headers
    res.set_header(header::ACCESS_CONTROL_ALLOW_ORIGIN,
                   HeaderValue::from_static("Any"));
    res.set_header(header::ACCESS_CONTROL_ALLOW_HEADERS,
                   HeaderValue::from_static("Origin, X-Requested-With, Content-Type, Accept"));

    // Pass control to the next middleware
    res.next_middleware()
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();
    server.utilize(enable_cors);
    server.options("**", middleware!(""));
    server.get("**", middleware!("Hello CORS Enabled World"));
    server.listen("127.0.0.1:6767").await.unwrap();
}
