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

#[test]
fn issue_326() {
    use hyper::Client;
    use hyper::header::Connection;

    // The issue is that if we have a body for the request, and it isn't read
    // by the server, it bleeds into the next request if keep-alive is active
    run_example("chaining", |port| {
        // The client is backed by a pool of keep-alive connections
        let client = Client::new();
        let url = format!("http://localhost:{}{}", port, "/post");

        // scope the requests so the connection gets released back to the pool
        {
            // Assert that keep alive is still true after a zero length request body
            // so that we know we're not dropping *all* requests with a body
            let ref mut res = client.post(&url)
                                    .send()
                                    .unwrap();
            assert_eq!(false, res.headers.has::<Connection>());
            assert_eq!("post", read_body_to_string(res));
        }

        {
            // Now send a request with a body, which won't be read by the server
            let ref mut res = client.post(&url)
                                    .body("0123456789")
                                    .send()
                                    .unwrap();
            // we could assert that `Connection: close` was given, but we may
            // want to be more flexible in future and allow buffers up to a
            // certain length to be drained.
            assert_eq!("post", read_body_to_string(res));
        }

        // Ensure the next request works as intended with no failure due to
        // a corrupted stream.
        let ref mut res = client.post(&url)
                                .body("0123456789")
                                .send()
                                .unwrap();
        assert_eq!("post", read_body_to_string(res));
    })
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
