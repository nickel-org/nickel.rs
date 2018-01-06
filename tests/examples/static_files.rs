use util::*;

use hyper::StatusCode;
use hyper::client::Response;

use std::io::{Read, Write};
use std::net::TcpStream;

fn with_path<F>(path: &str, f: F) where F: FnOnce(Response) {
    run_example("static_files", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        response_for(&url, f)
    })
}

#[test]
fn returns_expected_files() {
    with_path("/thoughtram_logo_brain.png", |res| {
        let status = res.status();
        assert_eq!(status, StatusCode::Ok);
        for_body(res, |head| {
            assert!(!&head.is_empty(), "no data for thoughtram_logo_brain.png")
        });
    });
}

#[test]
fn nested_files() {
    with_path("/nested/foo.js", |res| {
        for_body_as_string(res, |s| {
            assert!(s.starts_with("function foo"), "unexpected response: {:?}", s);
        });
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
        for_body_as_string(res, |s| {
            assert_eq!(s, "Not Found");
        });
    });
}
