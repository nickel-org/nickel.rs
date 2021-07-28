use super::with_path;
use crate::util::*;

use reqwest::blocking::Response;
use reqwest::StatusCode;

fn with_query<F>(query: &str, f: F) where F: FnOnce(Response) {
    with_path(&format!("/get?{}", query), f)
}

fn assert_accepted(query: &str) {
    with_query(query, |res| {
        assert_eq!(res.status(), StatusCode::OK);

        let s = read_body_to_string(res);
        assert_eq!(s, "Congratulations on conforming!");
    })
}

fn assert_rejected(query: &str) {
    with_query(query, |res| {
        assert_eq!(res.status(), StatusCode::BAD_REQUEST)
    })
}

mod accepts {
    use super::assert_accepted;

    #[test]
    fn valid() {
        assert_accepted("state=valid")
    }

    #[test]
    fn first_valid() {
        assert_accepted("state=valid&state=foo")
    }
}

mod rejects {
    use super::assert_rejected;

    #[test]
    fn invalid() {
        assert_rejected("state=foo")
    }

    #[test]
    fn other_keys() {
        assert_rejected("valid=valid")
    }

    #[test]
    fn empty() {
        assert_rejected("")
    }

    #[test]
    fn second_valid() {
        assert_rejected("state=foo&state=valid")
    }
}
