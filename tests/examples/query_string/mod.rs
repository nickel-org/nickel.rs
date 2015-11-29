use util::*;

use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("query_string", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let ref mut res = response_for(&url);
        f(res)
    })
}

mod get;
mod all;
