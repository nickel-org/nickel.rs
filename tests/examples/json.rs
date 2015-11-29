use util::*;

use hyper::client::Response;

fn with_path<F>(path: &str, f: F) where F: FnOnce(&mut Response) {
    run_example("json", |port| {
        let url = format!("http://localhost:{}{}", port, path);
        let ref mut res = response_for(&url);
        f(res)
    })
}

mod incoming {
    use util::*;

    use hyper::status::StatusCode;
    use hyper::client::Response;

    fn send_body<F>(body: &str, f: F) where F: FnOnce(&mut Response) {
        run_example("json", |port| {
            let url = format!("http://localhost:{}", port);
            let ref mut res = response_for_post(&url, body);
            f(res)
        })
    }

    #[test]
    fn parses_valid_requests() {
        let body = r#"{ "first_name": "Beautiful", "last_name": "World" }"#;
        send_body(body, |res| {
            let s = read_body_to_string(res);
            assert_eq!(s, "Hello Beautiful World");
        })
    }

    #[test]
    fn rejects_invalid() {
        // Missing 'firstname'
        let body = r#"{ "lastname": "World" }"#;
        send_body(body, |res| {
            assert_eq!(res.status, StatusCode::BadRequest);
        })
    }
}

mod outgoing {
    mod to_json {
        use super::super::with_path;
        use util::*;

        use rustc_serialize::json;

        use std::collections::HashMap;
        use hyper::{mime, header};

        #[test]
        fn serializes_valid_json() {
            with_path("/Pea/Nut", |res| {
                let s = read_body_to_string(res);
                let map: HashMap<String, String> = json::decode(&s).unwrap();
                assert_eq!(map["first_name"], "Pea");
                assert_eq!(map["last_name"], "Nut");
            })
        }

        #[test]
        fn sets_content_type_header() {
            with_path("/Pea/Nut", |res| {
                let content_type = res.headers.get::<header::ContentType>().unwrap();
                let expected: mime::Mime = "application/json".parse().unwrap();
                assert_eq!(content_type, &header::ContentType(expected));
            })
        }
    }

    mod raw {
        use super::super::with_path;
        use util::*;

        use rustc_serialize::json;

        use std::collections::HashMap;
        use hyper::{mime, header};

        #[test]
        fn serializes_valid_json() {
            with_path("/raw", |res| {
                let s = read_body_to_string(res);
                let map: HashMap<String, String> = json::decode(&s).unwrap();
                assert_eq!(map["foo"], "bar");
            })
        }

        #[test]
        fn sets_content_type_header() {
            with_path("/raw", |res| {
                let content_type = res.headers.get::<header::ContentType>().unwrap();
                let expected: mime::Mime = "application/json".parse().unwrap();
                assert_eq!(content_type, &header::ContentType(expected));
            })
        }
    }
}
