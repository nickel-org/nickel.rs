use util::*;

use hyper::status::StatusCode;
use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("error_handling", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let ref mut res = response_for(&url);
        f(res)
    })
}

#[test]
fn success_with_appresult() {
    with_path("/success", |response| {
        assert_eq!(response.status, StatusCode::Ok);
        let body = read_body_to_string(response);
        assert_eq!(body, "Everything went to plan!");
    })
}

#[test]
fn failure_with_appresult() {
    with_path("/failure", |response| {
        assert_eq!(response.status, StatusCode::BadRequest);
        // body gets set to "Bad Request" due to falling through to the default error handler
        let body = read_body_to_string(response);
        assert_eq!(body, "Bad Request");
    })
}

#[test]
fn custom_statuscode_from_json_failure() {
    for path in &["/json", "/json_nomacro"] {
        with_path(path, |response| {
            assert_eq!(response.status, StatusCode::ImATeapot);
            let body = read_body_to_string(response);
            assert_eq!(body, "<h2>I'm a Teapot!</h2>");
        });
    }
}
