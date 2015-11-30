use util::*;

use hyper::status::StatusCode;
use hyper::client::Response;

use std::io::{Read, Write};
use std::net::TcpStream;

fn with_path<F>(path: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("static_files", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let ref mut res = response_for(&url);
        f(res)
    })
}

#[test]
fn returns_expected_files() {
    with_path("/thoughtram_logo_brain.png", |res| {
        assert_eq!(res.status, StatusCode::Ok);
    });
}

#[test]
fn nested_files() {
    with_path("/nested/foo.js", |res| {
        let s = read_body_to_string(res);
        assert!(s.starts_with("function foo"), "unexpected response: {:?}", s);
    });
}

#[test]
fn rejects_parent_folder_access() {
    // we have to craft a raw request as hyper is good at parsing urls!
    run_example("static_files", |port| {
        let address = format!("localhost:{}", port);

        let mut stream = TcpStream::connect(&*address).unwrap();

        // send malicious request
        stream.write_all(b"GET /../../Cargo.toml HTTP/1.1\n\
                           Connection: close\n\n").unwrap();

        // assert status code is in the response headers
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        assert!(response.contains("400 Bad Request"), "Response was {:?}", response);
    })
}

#[test]
fn fallthroughs_with_same_prefix() {
    // depends on `works_with_another_middleware` passing
    with_path("/static/files/a", |res| {
        let s = read_body_to_string(res);
        assert_eq!(s, "Not Found");
    });
}
