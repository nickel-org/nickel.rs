use util::*;

use hyper::status::StatusCode;
use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("routing", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let ref mut res = response_for(&url);
        f(res)
    })
}

#[test]
fn concrete_path() {
    with_path("/bar", |res| {
        let s = read_body_to_string(res);
        assert_eq!(s, "This is the /bar handler");
    })
}

#[test]
fn parameterised_path() {
    with_path("/not_bar", |res| {
        let s = read_body_to_string(res);
        assert_eq!(s, "Foo is 'not_bar'. The requested format is ''");
    })
}

#[test]
fn single_wildcard_accept_single_directory() {
    with_path("/some/crazy/route", |res| {
        let s = read_body_to_string(res);
        assert_eq!(s, "This matches /some/crazy/route but not /some/super/crazy/route");
    })
}

#[test]
fn single_wildcard_reject_multi_directory() {
    with_path("/some/super/crazy/route", |res| {
        assert_eq!(res.status, StatusCode::NotFound);
    })
}

#[test]
fn double_wildcard() {
    let both = &["/a/crazy/route", "/a/super/crazy/route"];

    for path in both {
        with_path(path, |res| {
            let s = read_body_to_string(res);
            assert_eq!(s, "This matches /a/crazy/route and also /a/super/crazy/route");
        })
    }
}

#[test]
fn parameterised_path_with_format() {
    with_path("/not_bar.xml", |res| {
        let s = read_body_to_string(res);
        assert_eq!(s, "Foo is 'not_bar'. The requested format is 'xml'");
    })
}

#[test]
fn fallthrough_with_no_match() {
    with_path("/foo/bar", |res| {
        assert_eq!(res.status, StatusCode::NotFound);
    })
}
