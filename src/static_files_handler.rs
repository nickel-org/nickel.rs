use std::path::{AsPath, PathBuf};
use std::fs::PathExt;

use hyper::uri::RequestUri::AbsolutePath;
use hyper::method::Method::{Get, Head};

use request::Request;
use response::Response;
use middleware::{Halt, Continue, Middleware, MiddlewareResult};

// this should be much simpler after unboxed closures land in Rust.

#[derive(Clone)]
pub struct StaticFilesHandler {
    root_path: PathBuf
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
    pub fn new<P: AsPath>(root_path: P) -> StaticFilesHandler {
        StaticFilesHandler {
            root_path: root_path.as_path().to_path_buf()
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

    fn with_file<'a, 'b, P>(&self,
                            relative_path: Option<P>,
                            res: Response<'a>)
            -> MiddlewareResult<'a> where P: AsPath {
        if let Some(path) = relative_path {
            let path = self.root_path.join(path.as_path());
            if path.exists() {
                return Ok(Halt(try!(res.send_file(&path))));
            }
        };

        Ok(Continue(res))
    }
}
