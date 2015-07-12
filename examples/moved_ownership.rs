#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    let x = "Expensive computation".to_string();
    server.get("**", middleware!(&*x));
    server.listen("127.0.0.1:6767");
}
