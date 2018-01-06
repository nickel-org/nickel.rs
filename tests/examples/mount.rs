use util::*;

use hyper::StatusCode;
use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(Response) {
    run_example("mount", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        response_for(&url, f)
    })
}

#[test]
fn trims_the_prefix() {
    with_path("/test/foo", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "Got request with uri = '/foo'");
        });
    });

    with_path("/test/foo/bar.js", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "Got request with uri = '/foo/bar.js'");
        });
    })
}

#[test]
fn ignores_unmatched_prefixes() {
    with_path("/this_isnt_matched/foo", |res| {
        assert_eq!(res.status(), StatusCode::NotFound);
    })
}

#[test]
fn works_with_another_middleware() {
    with_path("/static/files/thoughtram_logo_brain.png", |res| {
        let status = res.status();
        assert_eq!(status, StatusCode::Ok);
        for_body(res, |head| {
            assert!(!&head.is_empty(), "no data for thoughtram_logo_brain.png")
        });
    });

    with_path("/static/files/nested/foo.js", |res| {
        for_body_as_string(res, |s| {
            assert!(s.starts_with("function foo"), "unexpected response: {:?}", s);
        });
    });
}

#[test]
fn fallthroughs_with_same_prefix() {
    // depends on `works_with_another_middleware` passing
    with_path("/static/files/a", |res| {
        for_body_as_string(res, |s| {
            assert_eq!(s, "No static file with path '/a'!");
        });
    });
}
