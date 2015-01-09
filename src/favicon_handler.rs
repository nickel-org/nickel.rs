use std::old_io::File;

use http::server::request::RequestUri::AbsolutePath;
use hyper::method::Method::{Get, Head, Options};
use hyper::status::StatusCode;

use request;
use response;
use middleware::{Action, Halt, Continue, Middleware};
use nickel_error::NickelError;
use mimes::MediaType;

pub struct FaviconHandler {
    icon: Vec<u8>,
    icon_path: Path, // Is it useful to log where in-memory favicon came from every request?
}

impl Middleware for FaviconHandler {
    fn invoke (&self, req: &mut request::Request, res: &mut response::Response)
               -> Result<Action, NickelError> {
        if FaviconHandler::is_favicon_request(req) {
            self.handle_request(req, res)
        } else {
            Ok(Continue)
        }
    }
}

impl FaviconHandler {
    /// Create a new middleware to serve an /favicon.ico file from an in-memory cache.
    /// The file is only read from disk once when the server starts.
    ///
    ///
    /// # Example
    /// ```{rust,ignore}
    /// use nickel::{Nickel, FaviconHandler};
    /// let mut server = Nickel::new();
    ///
    /// server.utilize(FaviconHandler::new("/path/to/ico/file"));
    /// ```
    pub fn new (icon_path: &str) -> FaviconHandler {
        let _icon_path = Path::new(icon_path);
        FaviconHandler {
            // Fail when favicon cannot be read. Better error message though?
            icon: File::open(&Path::new(icon_path)).unwrap().read_to_end().unwrap(),
            icon_path: _icon_path,
        }
    }

    #[inline]
    pub fn is_favicon_request (req: &request::Request) -> bool {
        match req.origin.request_uri {
            AbsolutePath(ref path) => &**path == "/favicon.ico",
            _                      => false
        }
    }

    pub fn handle_request (&self, req: &request::Request, res: &mut response::Response)
               -> Result<Action, NickelError> {
        match req.origin.method {
            Get | Head => {
                self.send_favicon(req, res);
            },
            Options => {
                res.status_code(StatusCode::Ok);
                res.origin.headers.allow = Some(vec!(Get, Head, Options));
            },
            _ => {
                res.status_code(StatusCode::MethodNotAllowed);
                res.origin.headers.allow = Some(vec!(Get, Head, Options));
            }
        }
        Ok(Halt)
    }

    pub fn send_favicon (&self, req: &request::Request, res: &mut response::Response) {
        debug!("{:?} {:?}", req.origin.method, self.icon_path.display());
        res.content_type(MediaType::Ico);
        res.send(&*self.icon);
    }
}
