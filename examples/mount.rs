#[macro_use] extern crate nickel;

use nickel::{Nickel, Mountable, StaticFilesHandler};

fn main() {
    let mut server = Nickel::new();

    /*
     * Requests to /t should not end up in this route. Expected output for
     * a request to /test/a is /a.
     */
    server.mount("/test/", middleware! { |req|
        format!("Got request with uri = '{}'", req.origin.uri)
    });

    /*
     * Fall-through behaviour, if StaticFilesHandler does not find a matching file,
     * the request uri must be reset so that it can be matched against other middleware.
     */
    server.mount("/static/files/", StaticFilesHandler::new("examples/assets/"));

    server.mount("/static/files/", middleware! { |req|
        let path = req.path_without_query().unwrap();
        format!("No static file with path '{}'!", path)
    });

    server.listen("127.0.0.1:6767");
}
