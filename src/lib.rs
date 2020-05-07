#![doc(test(attr(deny(warnings))))]

pub use hyper;

#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;

pub use crate::nickel::{Nickel, Options};
pub use crate::request::Request;
pub use crate::response::Response;
pub use crate::middleware::{Action, Continue, Halt, Middleware, ErrorHandler, MiddlewareResult};
//pub use crate::static_files_handler::StaticFilesHandler;
pub use crate::mount::{Mount, Mountable};
//pub use crate::favicon_handler::FaviconHandler;
pub use crate::default_error_handler::DefaultErrorHandler;
//pub use crate::body_parser::{BodyError, FormBody, JsonBody};
//pub use crate::query_string::QueryString;
pub use crate::urlencoded::{Params, Query};
pub use crate::router::{Router, Route, RouteResult, HttpRouter};
pub use crate::nickel_error::NickelError;
pub use crate::mimes::MediaType;
pub use crate::responder::Responder;
pub use crate::server::ListeningServer;
pub use crate::template_cache::{ReloadPolicy, TemplateCache};

#[macro_use] pub mod macros;

pub mod router;
mod server;
mod nickel;
mod request;
mod response;
mod middleware;
mod responder;
//mod favicon_handler;
//mod static_files_handler;
mod mount;
//mod body_parser;
//mod query_string;
pub mod mimes;
mod urlencoded;
mod nickel_error;
mod default_error_handler;
//pub mod extensions;
pub mod template_cache;

pub mod status {
    pub use hyper::StatusCode;
}
