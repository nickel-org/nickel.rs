use std::fs::File;
use std::path::{PathBuf, Path};
use std::io::Read;

use hyper::Method;
use hyper::StatusCode;
use hyper::header;

use crate::request::Request;
use crate::response::Response;
use crate::middleware::{Middleware, MiddlewareResult};
use crate::mimes::MediaType;

const ALLOWED_STR: &str = "GET, HEAD, OPTIONS";

pub struct FaviconHandler {
    icon: Vec<u8>,
    icon_path: PathBuf, // Is it useful to log where in-memory favicon came from every request?
}

impl<D> Middleware<D> for FaviconHandler {
    fn invoke<'a>(&'a self, req: &mut Request<'a, D>, res: Response<'a, D>)
            -> MiddlewareResult<'a, D> {
        if FaviconHandler::is_favicon_request(req) {
            self.handle_request(req, res)
        } else {
            res.next_middleware()
        }
    }
}

impl FaviconHandler {
    /// Create a new middleware to serve an /favicon.ico file from an in-memory cache.
    /// The file is only read from disk once when the server starts.
    ///
    /// # Examples
    /// ```{rust,no_run}
    /// use nickel::{Nickel, FaviconHandler};
    /// let mut server = Nickel::new();
    ///
    /// server.utilize(FaviconHandler::new("/path/to/ico/file"));
    /// ```
    pub fn new<P: AsRef<Path>>(icon_path: P) -> FaviconHandler {
        let icon_path = icon_path.as_ref().to_path_buf();
        let mut icon = vec![];
        File::open(&icon_path).unwrap().read_to_end(&mut icon).unwrap();

        FaviconHandler {
            // Fail when favicon cannot be read. Better error message though?
            icon: icon,
            icon_path: icon_path,
        }
    }

    #[inline]
    pub fn is_favicon_request<D>(req: &Request<'_, D>) -> bool {
        // Todo: migration cleanup
        // do we need to check req.origin.uri.is_absolute here?
        // would just req.origin.uri.path() work?
        req.origin.uri().path() == "/favicon.ico"
    }

    pub fn handle_request<'a, D>(&self, req: &Request<'_, D>, mut res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        match req.origin.method() {
            &Method::GET | &Method::HEAD => {
                self.send_favicon(req, res)
            },
            &Method::OPTIONS => {
                res.set(StatusCode::OK);
                res.set_header(header::ALLOW, header::HeaderValue::from_static(ALLOWED_STR));
                res.send("")
            },
            _ => {
                res.set(StatusCode::METHOD_NOT_ALLOWED);
                res.set_header(header::ALLOW, header::HeaderValue::from_static(ALLOWED_STR));
                res.send("")
            }
        }
    }

    pub fn send_favicon<'a, D>(&self, req: &Request<'_, D>, mut res: Response<'a, D>) -> MiddlewareResult<'a, D> {
        debug!("{:?} {:?}", req.origin.method(), self.icon_path.display());
        res.set(MediaType::Ico);
        res.send(&*self.icon)
    }
}
