use std::path::BytesContainer;
use std::str::from_utf8;

use http::server::request::AbsolutePath;

use request;
use response;
use middleware::MiddlewareHandler;

// this should be much simpler after unboxed closures land in Rust.

#[deriving(Clone)]
pub struct StaticFilesHandler {
    root_path: Path
}

impl MiddlewareHandler for StaticFilesHandler {
    fn invoke (&self, req: &mut request::Request, res: &mut response::Response) -> bool{
        match req.origin.request_uri {
            AbsolutePath(ref path) => {
                println!("GET {}{}.", from_utf8(self.root_path.container_as_bytes()).unwrap(), path);
                let mut relative_path = path.clone();
                if relative_path.eq(&"/".to_string()) {
                    relative_path = "index.html".to_string();
                } else {
                    relative_path.shift_char();
                }
                match res.send_file(&self.root_path.join(Path::new(relative_path.to_string()))) {
                    Ok(()) => false,
                    Err(_) => true
                }
            },
            _ => {
                true
            }
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
}