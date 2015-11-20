#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter};
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    let mut server = Nickel::new();

    // Here we'll use an 'atomic' counter, but any `Send`-able type should work.
    // If you want to mutate the data within a handler, you should wrap it
    // in either a `Mutex` or `RwLock`.
    let visits = AtomicUsize::new(0);

    // We wrap visits in an `Arc` to allow shared ownership between handlers
    let shared_visits = Arc::new(visits);

    // Make a shared clone of visits and move it to the handler for /a
    let visits_clone = shared_visits.clone();
    server.get("/a", middleware! {
        format!("{}", visits_clone.fetch_add(1, Relaxed))
    });

    // And finally we move the final shared instance into the handler for /b
    // (no need to clone the `Arc` again!)
    server.get("/b", middleware! {
        format!("{}", shared_visits.fetch_add(1, Relaxed))
    });

    server.listen("127.0.0.1:6767");
}
