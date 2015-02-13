#![feature(io)]

extern crate nickel;
extern crate http;

use nickel::{
    Nickel, Request, Response, HttpRouter
};
use std::old_io::net::ip::Ipv4Addr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

struct Logger {
    visits: AtomicUsize
}

fn main() {
    let mut server = Nickel::new();

    fn root_handler(_request: &Request, response: &mut Response, logger: &Logger) {
        response.send(format!("{}", logger.visits.fetch_add(1, Relaxed)));
    }

    // issue #20178
    let rhandler: fn(&Request, &mut Response, &Logger) = root_handler;

    server.get("/", (rhandler, Logger{visits: AtomicUsize::new(0)}));
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
