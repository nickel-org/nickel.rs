use std::io::File;

use http::server::request::AbsolutePath;
use http::method::{Get, Head, Options};
use http::status;

use request;
use response;
use middleware::{Action, Halt, Continue, Middleware};
use nickel_error::NickelError;

#[deriving(Clone)]
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
    pub fn new (icon_path: &str) -> FaviconHandler {
        let _icon_path = Path::new(icon_path);
        FaviconHandler {
            icon: File::open(&_icon_path).unwrap().read_to_end().unwrap(),
            icon_path: _icon_path,
            // Fail when favicon cannot be read. If you specify a favicon file,
            // but can't read it, that is a failure. Better error message though?
        }
    }

    #[inline]
    pub fn is_favicon_request (req: &request::Request) -> bool {
        match req.origin.request_uri { 
            AbsolutePath(ref path) => path.as_slice() == "/favicon.ico",
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
                res.status_code(status::Ok);
                res.origin.headers.allow = Some(vec!(Get, Head, Options));
            },
            _ => {
                res.status_code(status::MethodNotAllowed);
                res.origin.headers.allow = Some(vec!(Get, Head, Options));
            }
        }
        Ok(Halt)
    }

    pub fn send_favicon (&self, req: &request::Request, res: &mut response::Response) {
        println!("{} {}", req.origin.method, self.icon_path.display());
        res.content_type("ico");
        res.send(self.icon.clone());
    }
}
