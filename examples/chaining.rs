#[macro_use] extern crate nickel;
extern crate hyper;

use hyper::method::Method;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();
    server.add_route(Method::Get, "/", middleware!("Hello World"))
          .get("/get", middleware!("get"))
          .post("/post", middleware!("post"))
          .put("/put", middleware!("put"))
          .patch("/patch", middleware!("patch"))
          .delete("/delete", middleware!("delete"));

    server.listen("127.0.0.1:6767");
}
