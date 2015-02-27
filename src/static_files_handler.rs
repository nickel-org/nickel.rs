use std::old_io::{IoError, IoResult, FileNotFound};
use std::old_io::fs::PathExtensions;
use std::old_path::BytesContainer;
use std::error::FromError;

use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::{Get, Head};
use hyper::status::StatusCode::{InternalServerError, BadRequest};
use hyper::net;

use request::Request;
use response::Response;
use middleware::{Halt, Continue, Middleware, MiddlewareResult};
use nickel_error::{ NickelError, ErrorWithStatusCode };

// this should be much simpler after unboxed closures land in Rust.

#[derive(Clone)]
pub struct StaticFilesHandler {
    root_path: Path
}

impl Middleware for StaticFilesHandler {
    fn invoke<'a>(&self, req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
        match req.origin.method {
            Get | Head => self.with_file(self.extract_path(req), res),
            _ => Ok(Continue(res))
        }
    }
}

impl StaticFilesHandler {
    /// Create a new middleware to serve files from within a given root directory.
    /// The file to serve will be determined by combining the requested Url with
    /// the provided root directory.
    ///
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Nickel, StaticFilesHandler};
    /// let mut server = Nickel::new();
    ///
    /// server.utilize(StaticFilesHandler::new("/path/to/serve/"));
    /// ```
    pub fn new (root_path: &str) -> StaticFilesHandler {
        StaticFilesHandler {
            root_path: Path::new(root_path)
        }
    }

    fn extract_path<'a>(&self, req: &'a mut Request) -> Option<&'a str> {
        match req.origin.uri {
            AbsolutePath(ref path) => {
                debug!("{:?} {:?}{:?}", req.origin.method, self.root_path.display(), path);

                match &**path {
                    "/" => Some("index.html"),
                    path => Some(&path[1..]),
                }
            }
            _ => None
        }
    }

    fn with_file<'a, 'b, T>(&self,
                            relative_path: Option<T>,
                            res: Response<'a>)
            -> MiddlewareResult<'a> where T: BytesContainer {
        if let Some(path) = relative_path {
            let path = self.root_path.join(path);
            if path.exists() {
                return Ok(Halt(try!(res.send_file(&path))));
            }
        };

        Ok(Continue(res))
    }
}
