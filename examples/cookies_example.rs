#[macro_use] extern crate nickel;
extern crate cookie;

use nickel::{Nickel, HttpRouter, Cookies};
use nickel::cookies;

struct Data {
    secret_key: cookies::SecretKey
}

impl AsRef<cookies::SecretKey> for Data {
    fn as_ref(&self) -> &cookies::SecretKey {
        &self.secret_key
    }
}

fn main() {
    let data = Data { secret_key: cookies::SecretKey([0; 32]) };
    let mut server = Nickel::with_data(data);

    // Try curl -b MyCookie=bar localhost:6767
    server.get("/", middleware! { |req|
        let cookie = req.cookies().find("MyCookie");
        format!("MyCookie={:?}", cookie.map(|c| c.value))
    });

    server.listen("127.0.0.1:6767");
}
