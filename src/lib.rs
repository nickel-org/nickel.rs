#![crate_id = "floor#0.0.1"]
#![comment = "A expressjs inspired web framework for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![feature(macro_rules, phase)]


extern crate time;
extern crate http;
extern crate regex;
#[phase(plugin)]extern crate regex_macros;

pub use floor::Floor;
pub use request::Request;

mod router;
mod server;
mod floor;
mod request;
