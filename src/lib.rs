#![crate_name = "nickel"]
#![comment = "A expressjs inspired web framework for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![feature(macro_rules, phase)]

//!Nickel is supposed to be a simple and lightweight foundation for web applications written in Rust. Its API is inspired by the popular express framework for JavaScript.
//!
//!Some of the features are:
//!
//!* Easy handlers: A handler is just a function that takes a `Request` and `ResponseWriter`
//!* Variables in routes. Just write `my/route/:someid`
//!* Easy parameter access: `request.params.get(&"someid")`
//!* simple wildcard routes: `/some/*/route`
//!* double wildcard routes: `/a/**/route`
//!* middleware

extern crate time;
extern crate http;
extern crate serialize;
extern crate regex;
extern crate anymap;
extern crate url;
#[phase(plugin)]extern crate regex_macros;

pub use nickel::Nickel;
pub use request::Request;
pub use response::Response;
pub use middleware::{ Action, Continue, Halt, FromFn, Middleware };
pub use static_files_handler::StaticFilesHandler;
pub use json_body_parser::JsonBodyParser;
pub use query_string::QueryStringParser;
pub use nickel_error::{ NickelError, NickelErrorKind, Other };

mod router;
mod server;
mod nickel;
mod request;
mod response;
mod middleware;
mod static_files_handler;
mod json_body_parser;
mod mimes;
mod query_string;
mod urlencoded;
mod nickel_error;
