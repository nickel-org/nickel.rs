use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use std::collections::HashMap;

fn render(_req: &mut Request, res: Response) -> MiddlewareResult {
    let mut data = HashMap::<&str, &str>::new();
    data.insert("name", "user");
    return res.render("examples/assets/template.tpl", &data)
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    server.get("/", render);

    server.listen("127.0.0.1:6767").await.unwrap();
}
