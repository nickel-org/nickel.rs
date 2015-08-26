#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Cookies};

struct MyData;

fn main() {
    let mut server = Nickel::with_data(MyData);

    server.get("/", middleware! { |req|
        let cookie = req.cookies();
        //~^ ERROR: the trait `nickel::cookies::KeyProvider` is not implemented for the type `MyData`
        //~^^ ERROR: cannot infer an appropriate lifetime
    });

    server.get("/login", middleware! { |req, mut res|
        let jar = res.cookies_mut();
        //~^ ERROR: the trait `nickel::cookies::KeyProvider` is not implemented for the type `MyData`
    });

    server.listen("127.0.0.1:6767");
}
