use hyper::method::Method;
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};

use crate::util::{run_example, response_for_method, read_body_to_string};

#[test]
fn sets_headers() {
     run_example("enable_cors", |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        for path in &paths {
            let url = format!("http://localhost:{}/{}", port, path);
            let mut res = response_for_method(Method::Get, &url);

            assert_eq!(
                res.headers.get(),
                Some(&AccessControlAllowOrigin::Any)
            );

            assert_eq!(
                res.headers.get(),
                Some(&AccessControlAllowHeaders(vec![
                    "Origin".into(),
                    "X-Requested-With".into(),
                    "Content-Type".into(),
                    "Accept".into(),
                ]))
            );


            let body = read_body_to_string(&mut res);
            assert_eq!(body, "Hello CORS Enabled World");
        }
    })
}
