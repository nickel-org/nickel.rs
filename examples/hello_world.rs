#![feature(plugin, net)]
#[macro_use] extern crate nickel_macros;
extern crate nickel;

use nickel::Nickel;
use std::net::IpAddr;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen(IpAddr::new_v4(127, 0, 0, 1), 6767);
}
