use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::io::ErrorKind::NotFound;
use std::fs;

use hyper::Method;

use crate::status::StatusCode;
use crate::request::Request;
use crate::response::Response;
use crate::middleware::{Middleware, MiddlewareResult};

// this should be much simpler after unboxed closures land in Rust.

#[derive(Clone)]
pub struct StaticFilesHandler {
    root_path: PathBuf
}

#[async_trait]
impl<D: Send + 'static + Sync> Middleware<D> for StaticFilesHandler {
    async fn invoke(&self, req: &mut Request<D>, res: Response<D>)
            -> MiddlewareResult<D> {
        match *req.origin.method() {
            Method::GET | Method::HEAD => self.with_file(self.extract_path(req), res),
            _ => res.next_middleware()
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

    fn extract_path<'a, D>(&self, req: &'a mut Request<D>) -> &'a str {
        let path = req.path_without_query();
        debug!("{:?} {:?}{:?}", req.origin.method(), self.root_path.display(), path);
        
        match path {
            "/" => "index.html",
            path => &path[1..],
        }
    }

    fn with_file<D: Send + 'static + Sync, P>(&self,
                                              relative_path: P,
                                              res: Response<D>)
                                              -> MiddlewareResult<D> where P: AsRef<Path> {
        let path = relative_path.as_ref();
        if !safe_path(path) {
            let log_msg = format!("The path '{:?}' was denied access.", path);
            return res.error(StatusCode::BAD_REQUEST, log_msg);
        }
        
        let path = self.root_path.join(path);
        match fs::metadata(&path) {
            Ok(ref attr) if attr.is_file() => return res.send_file(&path),
            Err(ref e) if e.kind() != NotFound => debug!("Error getting metadata \
                                                          for file '{:?}': {:?}",
                                                         path, e),
            _ => {}
        };

        res.next_middleware()
    }
}

/// Block paths from accessing the parent directory
fn safe_path<P: AsRef<Path>>(path: P) -> bool {
    use std::path::Component;

    path.as_ref().components().all(|c| match c {
        // whitelist non-suspicious in case new things get added in future
        Component::CurDir | Component::Normal(_) => true,
        _ => false
    })
}

#[test]
fn bad_paths() {
    let bad_paths = &[
        "foo/bar/../baz/index.html",
        "foo/bar/../baz",
        "../bar/",
        "..",
        "/" // Root path should be handled already
    ];

    for &path in bad_paths {
        assert!(!safe_path(path), "expected {:?} to be suspicious", path);
    }
}

#[test]
fn valid_paths() {
    let good_paths = &[
        "foo/bar/./baz/index.html",
        "foo/bar/./baz",
        "./bar/",
        ".",
        "index.html"
    ];

    for &path in good_paths {
        assert!(safe_path(path), "expected {:?} to not be suspicious", path);
    }
}
