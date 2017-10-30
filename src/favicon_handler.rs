use std::fs::File;
use std::path::{PathBuf, Path};
use std::io::Read;

use hyper::Method::{Get, Head, Options};
use hyper::StatusCode;
use hyper::header;

use request::Request;
use response::Response;
use middleware::{Middleware, MiddlewareResult};
use mimes::MediaType;

pub struct FaviconHandler {
    icon: Vec<u8>,
    icon_path: PathBuf, // Is it useful to log where in-memory favicon came from every request?
}

impl<B, D> Middleware<B, D> for FaviconHandler {
    fn invoke<'a>(&'a self, req: &mut Request<'a, B, D>, res: Response<'a, B, D>)
            -> MiddlewareResult<'a, B, D> {
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
    pub fn is_favicon_request<B, D>(req: &Request<B, D>) -> bool {
        // Todo: migration cleanup
        // do we need to check req.origin.uri.is_absolute here?
        // would just req.origin.uri.path() work?
        req.origin.uri().path() == "/favicon.ico"
    }

    pub fn handle_request<'a, B, D>(&self, req: &Request<B, D>, mut res: Response<'a, B, D>) -> MiddlewareResult<'a, B, D> {
        match *req.origin.method() {
            Get | Head => {
                self.send_favicon(req, res)
            },
            Options => {
                res.set(StatusCode::Ok);
                res.set(header::Allow(vec![Get, Head, Options]));
                res.send("")
            },
            _ => {
                res.set(StatusCode::MethodNotAllowed);
                res.set(header::Allow(vec![Get, Head, Options]));
                res.send("")
            }
        }
    }

    pub fn send_favicon<'a, B, D>(&self, req: &Request<B, D>, mut res: Response<'a, B, D>) -> MiddlewareResult<'a, B, D> {
        debug!("{:?} {:?}", req.origin.method(), self.icon_path.display());
        res.set(MediaType::Ico);
        res.send(&*self.icon)
    }
}
