#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let mut server = Nickel::new();
    let visits = AtomicUsize::new(0);

    server.get("/", middleware! {
        format!("{}", visits.fetch_add(1, Relaxed))
    });
    server.listen("127.0.0.1:6767");
}
