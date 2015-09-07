#![doc(test(attr(deny(warnings))))]

extern crate time;
extern crate rustc_serialize as serialize;
extern crate hyper;
extern crate regex;
extern crate typemap;
extern crate plugin;
extern crate url;
extern crate mustache;
extern crate groupable;
extern crate modifier;
extern crate cookie;
extern crate byteorder;
extern crate rand;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

pub use nickel::Nickel;
pub use request::Request;
pub use response::Response;
pub use middleware::{Action, Continue, Halt, Middleware, ErrorHandler, MiddlewareResult};
pub use static_files_handler::StaticFilesHandler;
pub use mount::{Mount, Mountable};
pub use favicon_handler::FaviconHandler;
pub use default_error_handler::DefaultErrorHandler;
pub use json_body_parser::JsonBody;
pub use query_string::{QueryString, Query};
pub use router::{Router, Route, RouteResult, HttpRouter};
pub use nickel_error::NickelError;
pub use mimes::MediaType;
pub use responder::Responder;
pub use cookies::Cookies;

#[cfg(feature = "session")]
pub use session::{Session, CookieSession};

#[macro_use] pub mod macros;

pub mod router;
mod server;
mod nickel;
mod request;
mod response;
mod middleware;
mod responder;
mod favicon_handler;
mod static_files_handler;
mod mount;
mod json_body_parser;
pub mod mimes;
mod query_string;
mod urlencoded;
mod nickel_error;
mod default_error_handler;
pub mod cookies;
pub mod session;

pub mod status {
    pub use hyper::status::StatusCode;
}
