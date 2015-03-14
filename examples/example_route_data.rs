#![feature(net)]

extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult};
use std::net::IpAddr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

struct Logger {
    visits: AtomicUsize
}

fn main() {
    let mut server = Nickel::new();

    fn root_handler<'a>(_: &mut Request, response: Response<'a>, logger: &Logger)
            -> MiddlewareResult<'a> {
        let text = format!("{}", logger.visits.fetch_add(1, Relaxed));
        response.send(text)
    }

    // issue #20178
    let rhandler: for <'a> fn(&mut Request, Response<'a>, &Logger) -> MiddlewareResult<'a> = root_handler;

    server.get("/", (rhandler, Logger{visits: AtomicUsize::new(0)}));
    server.listen(IpAddr::new_v4(127, 0, 0, 1), 6767);
}
