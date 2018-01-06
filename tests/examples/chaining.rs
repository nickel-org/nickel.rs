use util::*;

use hyper::client::Response;
use hyper::Method;

fn with_path_and_method<F, S>(path: S, method: Method, f: F)
where F: Fn(Response),
      S: AsRef<str> {
    let method = method.clone();
    
    run_example("chaining", |port| {
        let url = format!("http://localhost:{}{}", port, path.as_ref());
        response_for_method(method, &url, f)
    })
}

mod expect_200 {
    use super::with_path_and_method;
    use util::*;
    use hyper::Method;
    use hyper::Method::*;

    #[test]
    fn root() {
        with_path_and_method("/", Get, |res| {
            for_body_as_string(res, |s| {
                assert_eq!(s, "Hello World");
            });
        });
    }

    macro_rules! test {
        ($($method:ident),+) => (
            $(
                #[test]
                fn $method() {
                    let paths = concat!("/", stringify!($method));
                    let method_str = stringify!($method);
                    let method: Method = method_str.to_uppercase().parse().unwrap();

                    println!("Method {:?}", method);
                    for path in &[paths] {
                        with_path_and_method(&path, method.clone(), |res| {
                            for_body_as_string(res, |s| {
                                assert_eq!(s, method_str);
                            });
                        });
                    }
                }
            )+
        )
    }

    test!(get, post, put, patch, delete);
}

mod expect_404 {
    use super::with_path_and_method;
    use hyper::Method;
    use hyper::Method::*;
    use hyper::StatusCode;

    static TEST_METHODS: &'static [Method] = &[Get, Post, Put, Patch, Delete];

    #[test]
    fn root() {
        let methods = TEST_METHODS.iter()
                                  .filter(|m| *m != &Method::Get);

        for method in methods {
            with_path_and_method("/", method.clone(), |res| {
                assert_eq!(res.status(), StatusCode::NotFound);
            })
        }
    }

    macro_rules! test {
        ($($method:ident),+) => (
            $(
                #[test]
                fn $method() {
                    let method_str = stringify!($method);
                    let method = method_str.to_uppercase().parse().unwrap();

                    // Don't test the endpoint that's actually for that method
                    let paths = TEST_METHODS.iter()
                                            .filter(|m| *m != &method)
                                            .map(|m| format!("/{}", m.to_string().to_lowercase()))
                                            .collect::<Vec<_>>();

                    for path in paths {
                        with_path_and_method(&path, method.clone(), |res| {
                            assert_eq!(res.status(), StatusCode::NotFound);
                        })
                    }
                }
            )+
        )
    }

    test!(get, post, put, patch, delete);
}
