#[macro_use] extern crate nickel;
extern crate cookie;

use nickel::{Nickel, HttpRouter, Cookies, QueryString};
use nickel::cookies;
use cookie::Cookie;

struct Data {
    secret_key: cookies::SecretKey
}

impl cookies::KeyProvider for Data {
    fn key(&self) -> cookies::SecretKey {
        self.secret_key.clone()
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

    // Note: Don't use get for login in real applications ;)
    // Try http://localhost:6767/login?name=foo
    server.get("/login", middleware! { |req, mut res|
        let jar = res.cookies_mut()
                     // long life cookies!
                     .permanent();

        let name = req.query().get("name")
                              .unwrap_or("default_name");
        let cookie = Cookie::new("MyCookie".to_owned(),
                                 name.to_owned());
        jar.add(cookie);

        "Cookie set!"
    });

    server.listen("127.0.0.1:6767");
}
