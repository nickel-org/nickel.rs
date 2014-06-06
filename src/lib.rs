#![crate_id = "floor#0.0.1"]
#![comment = "A expressjs inspired web framework for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate time;
extern crate http;
extern crate collections;

pub use floor::Floor;

mod routestore;
mod server;
mod floor;
