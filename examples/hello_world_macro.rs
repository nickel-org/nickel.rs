#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

#[tokio::main]
async fn main() {
    let mut server = Nickel::new();
    server.get("**", middleware!("Hello World"));
    server.listen("127.0.0.1:6767").await.unwrap();
}
