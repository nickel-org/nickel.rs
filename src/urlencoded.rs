use groupable::Groupable;
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::{Star, AbsoluteUri, AbsolutePath, Authority};
use std::collections::HashMap;
use url::{form_urlencoded, Url};

type QueryStore = HashMap<String, Vec<String>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Params(QueryStore);

// TODO: remove it in favor of Params
pub type Query = Params;

impl Params {
    /// Retrieves the first value from the query for `key`, or `None` if not present.
    ///
    /// # Notes
    /// There may be multiple values per key, if all of the values for a given
    /// `key` are required, then use `all`.
    //FIXME: Implement via Indexing whenever IndexGet is supported
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.first().map(|s| &**s))
    }

    /// Retrieve all values from the query for `key`, or `None` if none are present.
    pub fn all(&self, key: &str) -> Option<&[String]> {
        self.0.get(key).map(|v| &**v)
    }

    /// Retrieve the entire query map.
    pub fn map(&self) -> &HashMap<String, Vec<String>> {
        &self.0
    }
}

pub fn parse(encoded_string : &str) -> Params {
    Params(form_urlencoded::parse(encoded_string.as_bytes()).into_owned().group())
}

pub fn parse_uri(origin: &RequestUri) -> Params {
    let f = |query: Option<&str>| query.map(|q| parse(&*q));

    let result = match *origin {
        AbsoluteUri(ref url) => f(url.query()),
        AbsolutePath(ref s) => f(s.splitn(2, '?').nth(1)),
        Star | Authority(..) => None
    };

    result.unwrap_or_else(|| Params(HashMap::new()))
}

#[test]
fn parses_encoded_string_with_duplicate_keys() {
    let store = parse("foo=bar&message=hello&message=world");
    assert_eq!(
        store.all("foo"),
        Some(&["bar".to_string()][..])
    );
    assert_eq!(store.get("message"), Some("hello"));
    // Ensure the ordering is correct
    assert_eq!(
        store.all("message"),
        Some(&["hello".to_string(), "world".to_string()][..])
    );

    let map = store.map();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("foo"), Some(&vec!["bar".to_string()]));
    assert_eq!(
        map.get("message"),
        Some(&vec!["hello".to_string(), "world".to_string()])
    );
}

#[test]
fn parses_urlencoded_characters() {
    let store = parse("message=hello%20world");
    assert_eq!(store.get("message"), Some("hello world"));

    let map = store.map();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("message"), Some(&vec!["hello world".to_string()]));
}

#[test]
fn splits_and_parses_an_url() {
    use url::Url;
    let t = |url| {
        let store = parse_uri(&url);
        assert_eq!(store.get("foo"), Some("bar"));
        assert_eq!(store.get("foo").unwrap_or("other"), "bar");
        assert_eq!(store.get("bar").unwrap_or("other"), "other");
        assert_eq!(store.all("message"),
                        Some(&["hello".to_string(), "world".to_string()][..]));
        assert_eq!(store.all("car"), None);
    };

    let raw = "http://www.foo.bar/query/test?foo=bar&message=hello&message=world";
    t(AbsoluteUri(Url::parse(raw).unwrap()));

    t(AbsolutePath("/query/test?foo=bar&message=hello&message=world".to_string()));

    assert_eq!(parse_uri(&Star), Params(HashMap::new()));

    let store = parse_uri(&Authority("host.com".to_string()));
    assert_eq!(store, Params(HashMap::new()));
}
