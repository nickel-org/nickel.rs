#![feature(plugin, old_io)]
#[macro_use] extern crate nickel_macros;
extern crate nickel;

use nickel::Nickel;
use std::old_io::net::ip::Ipv4Addr;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
