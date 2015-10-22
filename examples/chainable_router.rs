#[macro_use] extern crate nickel;
extern crate hyper;

use nickel::{Nickel};
use nickel::router::chainable_router::{ChainablePath, ChainableHandler};

fn main() {
    let mut server = Nickel::new();
    server
        .route("/")
        .get(middleware!("get"))
        .post(middleware!("post"))
        .put(middleware!("put"))
        .patch(middleware!("patch"))
        .delete(middleware!("delete"))
        .route("/chain1")
        .get(middleware!("chain1"))
        .route("/chain2")
        .get(middleware!("chain2"));

    server.listen("127.0.0.1:6767");
}
