//! This example only works if you enable the `session` feature.
#![cfg_attr(not(feature = "session"), allow(dead_code, unused_imports))]

#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate time;

use std::io::Write;
use nickel::*;
use nickel::status::StatusCode;
use time::Duration;

#[derive(RustcDecodable, RustcEncodable)]
struct User {
    name: String,
    password:  String,
}

struct ServerData;
static SECRET_KEY: &'static cookies::SecretKey = &cookies::SecretKey([0; 32]);

#[cfg(feature = "session")]
impl cookies::KeyProvider for ServerData {
    fn key(&self) -> cookies::SecretKey { SECRET_KEY.clone() }
}

#[cfg(feature = "session")]
impl session::Store for ServerData {
    type Session = Option<String>;

    fn timeout() -> Duration {
        Duration::seconds(5)
    }
}

#[cfg(not(feature = "session"))]
fn main() {}

#[cfg(feature = "session")]
fn main() {
    let mut server = Nickel::with_data(ServerData);

    // Anyone should be able to reach this route
    server.get("/", middleware! { |req, mut res|
        format!("You are logged in as: {:?}\n", CookieSession::get_mut(req, &mut res))
    });

    server.post("/login", middleware!{|req, mut res|
        if let Ok(u) = req.json_as::<User>() {
            if u.name == "foo" && u.password == "bar" {
                *CookieSession::get_mut(req, &mut res) = Some(u.name);
                return res.send("Successfully logged in.")
            }
        }
        (StatusCode::BadRequest, "Access denied.")
    });

    server.get("/secret", middleware! { |req, mut res| <ServerData>
        match *CookieSession::get_mut(req, &mut res) {
            Some(ref user) if user == "foo" => (StatusCode::Ok, "Some hidden information!"),
            _ => (StatusCode::Forbidden, "Access denied.")
        }
    });

    fn custom_403<'a>(err: &mut NickelError<ServerData>, _: &mut Request<ServerData>) -> Action {
        if let Some(ref mut res) = err.stream {
            if res.status() == StatusCode::Forbidden {
                let _ = res.write_all(b"Access denied!\n");
                return Halt(())
            }
        }

        Continue(())
    }

    // issue #20178
    let custom_handler: fn(&mut NickelError<ServerData>, &mut Request<ServerData>) -> Action = custom_403;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767");
}
