#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn enable_cors<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    // Set appropriate headers
    res.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Methods", vec![b"*".to_vec()]);
    res.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin, X-Requested-With, Content-Type, Accept".to_vec()]);

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
