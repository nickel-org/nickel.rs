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
#[phase(plugin)]
extern crate regex_macros;

pub use nickel::Nickel;
pub use request::Request;
pub use response::Response;
pub use middleware::{ Action, Continue, Halt, Middleware, ErrorHandler };
pub use static_files_handler::StaticFilesHandler;
pub use default_error_handler::DefaultErrorHandler;
pub use json_body_parser::{JsonBodyParser, JsonBody};
pub use query_string::{QueryStringParser, QueryString};
pub use router::{ Router, Route, RouteResult };
pub use nickel_error::{ NickelError, NickelErrorKind, ErrorWithStatusCode, UserDefinedError, Other };
pub use into_middleware::IntoMiddleware;
pub use into_error_handler::IntoErrorHandler;
pub use mimes::get_media_type;

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
mod default_error_handler;
mod into_middleware;
mod into_error_handler;
