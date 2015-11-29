#[macro_use] extern crate nickel;

use nickel::status::StatusCode;
use nickel::{Nickel, QueryString, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    // try calling http://localhost:6767/all?foo=bar&foo=car
    server.get("/all", middleware! { |request|
        if let Some(vals) = request.query().all("foo") {
            format!("Your foo values in the query string are: {:?}", vals)
        } else {
            format!("You didn't provide any foo values!")
        }
    });

    // try calling http://localhost:6767/get?state=valid
    // then try calling http://localhost:6767/get?state=invalid
    server.get("/get", middleware! { |request|
        if request.query().get("state") != Some("valid") {
            (StatusCode::BadRequest, "State parameter was not valid")
        } else {
            (StatusCode::Ok, "Congratulations on conforming!")
        }
    });

    server.listen("127.0.0.1:6767");
}
