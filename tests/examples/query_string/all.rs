use super::with_path;
use util::*;

use hyper::client::Response;

fn with_query<F>(query: &str, f: F) where F: FnOnce(&mut Response) {
    with_path(&format!("/all?{}", query), f)
}

fn assert_accepted<'a, T>(query: &str, expected_values: T)
where T: AsRef<[&'a str]> {
    with_query(query, |res| {
        assert_eq!(
            read_body_to_string(res),
            format!("Your foo values in the query string are: {:?}",
                    expected_values.as_ref())
        );
    })
}

fn assert_rejected(query: &str) {
    with_query(query, |res| {
        assert_eq!(
            read_body_to_string(res),
            "You didn't provide any foo values!"
        );
    })
}

mod accepts {
    mod one {
        use super::super::assert_accepted;

        #[test]
        fn with_value() {
            assert_accepted("foo=bar", ["bar"])
        }

        #[test]
        fn no_value() {
            assert_accepted("foo", [""])
        }

        #[test]
        fn ignores_other_keys() {
            assert_accepted("bar=foo&foo=car&car=bar", ["car"])
        }

        #[test]
        fn ignores_other_keys_without_value() {
            assert_accepted("bar=foo&foo&car=bar", [""])
        }
    }

    mod many {
        use super::super::assert_accepted;

        #[test]
        fn with_value() {
            assert_accepted("foo=bar&foo=car", ["bar", "car"])
        }

        #[test]
        fn duplicate_values() {
            assert_accepted("foo=bar&foo=car&foo=bar", ["bar", "car", "bar"])
        }

        #[test]
        fn no_value() {
            assert_accepted("foo&foo", ["", ""])
        }

        #[test]
        fn ignores_other_keys() {
            assert_accepted("bar=foo&foo&car=bar&foo=car", ["", "car"])
        }
    }
}

mod rejects {
    use super::assert_rejected;

    #[test]
    fn other_keys() {
        assert_rejected("bar=foo")
    }

    #[test]
    fn empty() {
        assert_rejected("")
    }
}
