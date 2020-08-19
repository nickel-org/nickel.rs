use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn hello_world(_req: &mut Request, res: Response) -> MiddlewareResult {
    res.send("Hello World")
}

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();
    server.get("**", hello_world);
    server.listen("127.0.0.1:6767").await.unwrap();
}
