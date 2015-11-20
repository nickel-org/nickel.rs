use util::*;

use hyper::client::Response;
use hyper::method::Method;

fn with_paths_and_method<F, S>(paths: &[S], method: Method, f: F)
where F: Fn(&mut Response),
      S: AsRef<str> {
    for path in paths {
        let method = method.clone();

        run_example("chaining", |port| {
            let url = format!("http://localhost:{}{}", port, path.as_ref());
            let ref mut res = response_for_method(method, &url);
            f(res)
        })
    }
}

mod expect_200 {
    use super::with_paths_and_method;
    use util::*;
    use hyper::method::Method::*;

    #[test]
    fn root() {
        with_paths_and_method(&["/"], Get, |res| {
            let s = read_body_to_string(res);
            assert_eq!(s, "Hello World");
        });
    }

    macro_rules! test {
        ($($method:ident),+) => (
            $(
                #[test]
                fn $method() {
                    let path = concat!("/", stringify!($method));
                    let method_str = stringify!($method);
                    let method = method_str.to_uppercase().parse().unwrap();

                    println!("Method {:?}", method);
                    with_paths_and_method(&[path], method, |res| {
                        let s = read_body_to_string(res);
                        assert_eq!(s, method_str);
                    });
                }
            )+
        )
    }

    test!(get, post, put, patch, delete);
}

mod expect_404 {
    use super::with_paths_and_method;
    use hyper::method::Method;
    use hyper::method::Method::*;
    use hyper::status::StatusCode;

    static TEST_METHODS: &'static [Method] = &[Get, Post, Put, Patch, Delete];

    #[test]
    fn root() {
        let methods = TEST_METHODS.iter()
                                  .filter(|m| *m != &Method::Get);

        for method in methods {
            with_paths_and_method(&["/"], method.clone(), |res| {
                assert_eq!(res.status, StatusCode::NotFound);
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

                    with_paths_and_method(&paths, method, |res| {
                        assert_eq!(res.status, StatusCode::NotFound);
                    })
                }
            )+
        )
    }

    test!(get, post, put, patch, delete);
}
