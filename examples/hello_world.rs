use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn hello_world<'mw>(_req: &mut Request<'_, '_>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.send("Hello World")
}

fn main() {
    let mut server = Nickel::new();
    server.get("**", hello_world);
    server.listen("127.0.0.1:6767").unwrap();
}
