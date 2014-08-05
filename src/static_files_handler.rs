use std::path::BytesContainer;
use std::str::from_utf8;
use std::io::{IoError, IoResult, FileNotFound};

use http::server::request::AbsolutePath;
use http::method::{Get, Head};
use http::status::NotFound;

use request;
use response;
use middleware::{Action, Halt, Continue, Middleware};

// this should be much simpler after unboxed closures land in Rust.

#[deriving(Clone)]
pub struct StaticFilesHandler {
    root_path: Path
}

impl Middleware for StaticFilesHandler {
    fn invoke (&self, req: &mut request::Request, res: &mut response::Response) -> Action {
        match req.origin.method {
            Get | Head => {
                match self.with_file(self.extract_path(req), res) {
                    Ok(()) => Halt,
                    Err(err) => match err.kind {
                        FileNotFound => {
                            res.origin.status = NotFound;
                            Continue
                        },
                        _ => Continue
                    }
                }
            },
            _ => Continue
        }
    }
}

impl StaticFilesHandler {
    pub fn new (root_path: &str) -> StaticFilesHandler {
        let checked_path = Path::new(root_path);
        StaticFilesHandler {
            root_path: checked_path
        }
    }

    fn extract_path(&self, req: &mut request::Request) -> Option<String> {
        match req.origin.request_uri {
            AbsolutePath(ref path) => {
                println!("{} {}{}",req.origin.method, from_utf8(self.root_path.container_as_bytes()).unwrap(), path);
                let mut relative_path = path.clone();
                if relative_path.eq(&"/".to_string()) {
                    relative_path = "index.html".to_string();
                } else {
                    relative_path.shift_char();
                }
                Some(relative_path)
            }
            _ => None
        }
    }

    fn with_file(&self, relative_path: Option<String>, res: &mut response::Response) -> IoResult<()> {
        match relative_path {
            Some(path) => res.send_file(&self.root_path.join(Path::new(path))),
            None => Err(IoError::last_error())
        }
    }
}
