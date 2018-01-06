use util::*;

use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(Response) {
    run_example("query_string", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        response_for(&url, f);
    })
}

mod get;
mod all;
