use crate::util::*;

use reqwest::blocking::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(Response) {
    run_example("query_string", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let res = response_for(&url);
        f(res)
    })
}

mod get;
mod all;
