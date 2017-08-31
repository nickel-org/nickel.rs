#![doc(test(attr(allow(warnings))))]

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
extern crate futures;
extern crate tokio_core;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

pub use nickel::{Nickel, Options};
pub use request::Request;
pub use response::Response;
pub use middleware::{Action, Continue, Halt, Middleware, ErrorHandler, MiddlewareResult};
pub use static_files_handler::StaticFilesHandler;
pub use mount::{Mount, Mountable};
pub use favicon_handler::FaviconHandler;
pub use default_error_handler::DefaultErrorHandler;
pub use body_parser::{BodyError, FormBody, JsonBody};
pub use query_string::QueryString;
pub use urlencoded::{Params, Query};
pub use router::{Router, Route, RouteResult, HttpRouter};
pub use nickel_error::NickelError;
pub use mimes::MediaType;
pub use responder::Responder;
pub use server::ListeningServer;

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
mod body_parser;
mod query_string;
pub mod mimes;
mod urlencoded;
mod nickel_error;
mod default_error_handler;
pub mod extensions;

pub mod status {
    pub use hyper::StatusCode;
}
