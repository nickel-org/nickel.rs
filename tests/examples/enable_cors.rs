use hyper::Method;
use hyper::header::{AccessControlAllowOrigin, AccessControlAllowHeaders};
use unicase::Ascii;

use util::{run_example, response_for_method, for_body_as_string};

#[test]
fn sets_headers() {
     run_example("enable_cors", |port| {
        let paths = ["", "foo", "bar.html", "foo-barrrr/baz"];

        for path in &paths {
            let url = format!("http://localhost:{}/{}", port, path);
            response_for_method(Method::Get, &url, |res| {

                assert_eq!(
                    res.headers().get(),
                    Some(&AccessControlAllowOrigin::Any)
                );
                
                assert_eq!(
                    res.headers().get(),
                    Some(&AccessControlAllowHeaders(vec![
                        Ascii::new("Origin".to_owned()),
                        Ascii::new("X-Requested-With".to_owned()),
                        Ascii::new("Content-Type".to_owned()),
                        Ascii::new("Accept".to_owned()),
                    ]))
                );
                
                for_body_as_string(res, |body| {assert_eq!(body, "Hello CORS Enabled World")});
            });
        }
    })
}
