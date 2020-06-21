use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

struct MyConfig {
    greet: String,
}

fn greeter<'mw>(req: &mut Request<'_, MyConfig>, res: Response<'mw, MyConfig>) -> MiddlewareResult<'mw, MyConfig> {
    let my_config = req.server_data();
    res.send(&*my_config.greet)
}

#[tokio::main]
async fn main() {
    let my_config = MyConfig { greet: "hello".to_string() };

    let mut server = Nickel::with_data(my_config);
    server.get("**", greeter);
    server.listen("127.0.0.1:6767").await.unwrap();
}
