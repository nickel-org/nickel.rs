#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use std::collections::HashMap;

fn main() {
    let mut server = Nickel::new();

    // * NOTE *
    // This example is deprecated, you should use nickel_mustache from crates.io
    server.get("/", middleware! { |_, res|
        let mut data = HashMap::<&str, &str>::new();
        data.insert("name", "user");
        return res.render("examples/assets/template.tpl", &data)
    });

    server.listen("127.0.0.1:6767");
}
