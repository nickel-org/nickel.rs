use std::old_path::BytesContainer;
use std::old_io::{IoError, IoResult, FileNotFound};

use http::server::request::RequestUri::AbsolutePath;
use http::method::{Get, Head};
use http::status::{ InternalServerError };

use request;
use response;
use middleware::{Halt, Continue, Middleware, MiddlewareResult};
use nickel_error::{ NickelError, ErrorWithStatusCode };

// this should be much simpler after unboxed closures land in Rust.

#[derive(Clone)]
pub struct StaticFilesHandler {
    root_path: Path
}

impl Middleware for StaticFilesHandler {
    fn invoke (&self, req: &mut request::Request, res: &mut response::Response)
               -> MiddlewareResult {
        match req.origin.method {
            Get | Head => {
                match self.with_file(self.extract_path(req), res) {
                    Ok(()) => Ok(Halt),
                    Err(err) => match err.kind {
                        // We shouldn't assume the StaticFileHandler to be the last middleware in the stack.
                        // Therefore it's important to continue in case of FileNotFound errors.
                        FileNotFound => Ok(Continue),
                        _ => Err(NickelError::new(format!("Unknown Error ({})", err),
                                                  ErrorWithStatusCode(InternalServerError)))
                    }
                }
            },
            _ => Ok(Continue)
        }
    }
}

impl StaticFilesHandler {
    /// Create a new middleware to serve files from within a given root directory.
    /// The file to serve will be determined by combining the requested Url with
    /// the provided root directory.
    ///
    ///
    /// # Example
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

    fn extract_path<'a>(&self, req: &'a mut request::Request) -> Option<&'a str> {
        match req.origin.request_uri {
            AbsolutePath(ref path) => {
                debug!("{:?} {:?}{:?}", req.origin.method, self.root_path.display(), path);

                match &path[] {
                    "/" => Some("index.html"),
                    path => Some(&path[1..]),
                }
            }
            _ => None
        }
    }

    fn with_file<T: BytesContainer>(&self, relative_path: Option<T>, res: &mut response::Response)
                                    -> IoResult<()> {
        match relative_path {
            Some(path) => res.send_file(&self.root_path.join(path)),
            None => Err(IoError::last_error())
        }
    }
}
