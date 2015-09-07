#[macro_use] extern crate nickel;
extern crate cookie;

use nickel::{Nickel, HttpRouter, Cookies, QueryString};
use cookie::Cookie;

fn main() {
    let mut server = Nickel::new();

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

    // Try `curl -c /tmp/cookie -b /tmp/cookie http://localhost:6767/secure?value=foobar`
    // when the `secure_cookies` feature is enabled
    // i.e. `cargo run --example cookies_example --features secure_cookies
    if cfg!(feature = "secure_cookies") {
        server.get("/secure", middleware! { |req, mut res|
            let jar = res.cookies_mut()
                         .encrypted();

            let new_value = req.query().get("value")
                                   .unwrap_or("no value")
                                   .to_owned();

            let cookie = Cookie::new("SecureCookie".to_owned(),
                                     new_value);
            jar.add(cookie);

            // Old value from the request's Cookies
            let old_value = req.cookies()
                               .encrypted()
                               .find("SecureCookie")
                               .map(|c| c.value);

            format!("Old value was {:?}", old_value)
        });
    }

    server.listen("127.0.0.1:6767");
}
