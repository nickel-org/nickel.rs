use async_trait::async_trait;
use nickel::{Nickel, HttpRouter, Request, Response, Middleware, MiddlewareResult};
use std::collections::HashMap;

// TODO: we don't have a default Middleware impl for async functions like this,
// so we need to implement the trait as below.
//
// async fn render(_req: &mut Request, res: Response) -> MiddlewareResult {
//     let mut data = HashMap::<&str, &str>::new();
//     data.insert("name", "user");
//     return res.render("examples/assets/template.tpl", &data).await
// }

struct Render;

#[async_trait]
impl Middleware<()> for Render {
    async fn invoke(&self, _req: &mut Request, res: Response)
    -> MiddlewareResult {
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "user");
        return res.render("examples/assets/template.tpl", &data).await
    }
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();

    server.get("/", Render);

    server.listen("127.0.0.1:6767").await.unwrap();
}
