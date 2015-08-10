use std::path::{Path, PathBuf};
use std::io::ErrorKind::NotFound;
use std::fs;

use hyper::method::Method::{Get, Head};

use request::Request;
use response::Response;
use middleware::{Continue, Middleware, MiddlewareResult};

// this should be much simpler after unboxed closures land in Rust.

#[derive(Clone)]
pub struct StaticFilesHandler {
    root_path: PathBuf
}

impl<D> Middleware<D> for StaticFilesHandler {
    fn invoke<'a, 'k>(&self, res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
        match res.request.origin.method {
            Get | Head => self.handle(res),
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
    pub fn new<P: AsRef<Path>>(root_path: P) -> StaticFilesHandler {
        StaticFilesHandler {
            root_path: root_path.as_ref().to_path_buf()
        }
    }

    fn extract_path<'a>(&self, req: &'a Request) -> Option<&'a str> {
        req.path_without_query().map(|path| {
            debug!("{:?} {:?}{:?}", req.origin.method, self.root_path.display(), path);

            match path {
                "/" => "index.html",
                path => &path[1..],
            }
        })
    }

    fn handle<'a, 'k, D>(&self, res: Response<'a, 'k, D>) -> MiddlewareResult<'a, 'k, D> {
        if let Some(path) = self.extract_path(&res.request).map(|p| self.root_path.join(p)) {
            match fs::metadata(&path) {
                Ok(ref attr) if attr.is_file() => return res.send_file(&path),
                Err(ref e) if e.kind() != NotFound => debug!("Error getting metadata \
                                                              for file '{:?}': {:?}",
                                                              path, e),
                _ => {}
            }
        };

        Ok(Continue(res))
    }
}
