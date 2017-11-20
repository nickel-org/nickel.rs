extern crate nickel;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

fn render<'mw, 'conn>(_req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user");
    return res.render("examples/assets/template.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    server.get("/", render);

    server.listen("127.0.0.1:6767").unwrap();
}
