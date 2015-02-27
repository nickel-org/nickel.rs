#![feature(old_io)]

extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, Halt};
use std::old_io::net::ip::Ipv4Addr;
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
        let response = try!(response.send(text));
        Ok(Halt(response))
    }

    // issue #20178
    let rhandler: for <'a> fn(&mut Request, Response<'a>, &Logger) -> MiddlewareResult<'a> = root_handler;

    server.get("/", (rhandler, Logger{visits: AtomicUsize::new(0)}));
    server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
}
