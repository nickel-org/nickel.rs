use util::*;

use hyper::StatusCode;
use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(Response) {
    run_example("regex_route", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        response_for(&url, f);
    })
}

#[test]
fn with_param() {
    with_path("/hello/world", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "Hello world");
        });
    })
}

#[test]
fn ignores_query() {
    with_path("/hello/world?foo=bar", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "Hello world");
        });
    })
}

#[test]
// FIXME?
// Rym: I would expect this to 404, but its behavior is somewhat
// expected when compared to the regex provided. To get my expected
// behaviour, you'd need to append `$` to the regex in the example.
// This seems like it might be a bit of a footgun.
fn fallthrough_too_many_params() {
    with_path("/hello/beautiful/world", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "Hello beautiful");
        });
    })
}

#[test]
fn fallthrough_with_no_match() {
    with_path("/", |res| {
        assert_eq!(res.status(), StatusCode::NotFound);
    })
}
