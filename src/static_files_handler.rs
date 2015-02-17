use std::old_io::{IoError, IoResult, FileNotFound};
use std::old_path::BytesContainer;

use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::{Get, Head};
use hyper::status::StatusCode::InternalServerError;
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
    fn invoke<'a, 'b>(&self, req: &mut Request, res: Response<'a, 'b>)
               -> MiddlewareResult<'a, 'b> {
        match req.origin.method {
            Get | Head => {
                match self.with_file(self.extract_path(req), res) {
                    Ok(stream) => Ok(Halt(stream)),
                    Err(_) => panic!(),
                    // Err(err) => match err.kind {
                    //     // We shouldn't assume the StaticFileHandler to be the last middleware in the stack.
                    //     // Therefore it's important to continue in case of FileNotFound errors.
                    //     FileNotFound => Ok(Continue),
                    //     _ => Err(NickelError::new(format!("Unknown Error ({})", err),
                    //                               ErrorWithStatusCode(InternalServerError)))
                    // }
                }
            },
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

    fn with_file<'a, 'b, T: BytesContainer>(&self, relative_path: Option<T>, res: Response<'a, 'b>)
                                    -> IoResult<Response<'a, 'b, net::Streaming>> {
        match relative_path {
            Some(path) => res.send_file(&self.root_path.join(path)),
            None => Err(IoError::last_error())
        }
    }
}
