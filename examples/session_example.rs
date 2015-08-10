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
impl AsRef<cookies::SecretKey> for ServerData {
    fn as_ref(&self) -> &cookies::SecretKey { SECRET_KEY }
}
impl SessionStore for ServerData {
    type Store = Option<String>;

    fn timeout() -> Duration {
        Duration::seconds(5)
    }
}


fn main() {
    let mut server = Nickel::with_data(ServerData);

    /* Anyone should be able to reach thist route. */
    server.get("/", middleware! { |mut res|
        format!("You are logged in as: {:?}\n", res.session())
    });

    server.post("/login", middleware!{|mut res|
        if let Ok(u) = res.request.json_as::<User>() {
            if u.name == "foo" && u.password == "bar" {
                *res.session_mut() = Some(u.name);
                return res.send("Successfully logged in.")
            }
        }
        (StatusCode::BadRequest, "Access denied.")
    });

    server.get("/secret", middleware! { |mut res| <ServerData>
        match *res.session() {
            Some(ref user) if user == "foo" => (StatusCode::Ok, "Some hidden information!"),
            _ => (StatusCode::Forbidden, "Access denied.")
        }
    });

    fn custom_403<'a>(err: &mut NickelError<ServerData>) -> Action {
        if let Some(ref mut res) = err.response_mut() {
            if res.status() == StatusCode::Forbidden {
                let _ = res.write_all(b"Access denied!\n");
                return Halt(())
            }
        }

        Continue(())
    }

    // issue #20178
    let custom_handler: fn(&mut NickelError<ServerData>) -> Action = custom_403;

    server.handle_error(custom_handler);

    server.listen("127.0.0.1:6767");
}
