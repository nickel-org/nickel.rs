use reqwest::Method;
use reqwest::header::{ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_ALLOW_HEADERS};

use crate::util::{run_example, response_for_method, read_body_to_string};

#[test]
fn sets_headers() {
     run_example("enable_cors", |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        for path in &paths {
            let url = format!("http://localhost:{}/{}", port, path);
            let mut res = response_for_method(Method::GET, &url);

            assert_eq!(
                res.headers().get(ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
                "Any"
            );

            assert_eq!(
                res.headers().get(ACCESS_CONTROL_ALLOW_HEADERS).unwrap(),
                "Origin, X-Requested-With, Content-Type, Accept"
            );


            let body = read_body_to_string(res);
            assert_eq!(body, "Hello CORS Enabled World");
        }
    })
}
