use std::collections::HashMap;
use request::Request;
use urlencoded;
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::{Star, AbsoluteUri, AbsolutePath, Authority};
use url::UrlParser;
use plugin::{Plugin, Pluggable};
use typemap::Key;

type QueryStore = HashMap<String, Vec<String>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Query(QueryStore);

impl Query {
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
}

// Plugin boilerplate
struct QueryStringParser;
impl Key for QueryStringParser { type Value = Query; }

impl<'a, 'k> Plugin<Request<'a, 'k>> for QueryStringParser {
    type Error = ();

    fn eval(req: &mut Request) -> Result<Query, ()> {
        Ok(parse(&req.origin.uri))
    }
}

pub trait QueryString {
    /// Retrieve the query from the current `Request`.
    fn query(&mut self) -> &Query;
}

impl<'a, 'k> QueryString for Request<'a, 'k> {
    fn query(&mut self) -> &Query {
        self.get_ref::<QueryStringParser>()
            .ok()
            .expect("Bug: QueryStringParser returned None")
    }
}

fn parse(origin: &RequestUri) -> Query {
    let f = |query: Option<&String>| query.map(|q| urlencoded::parse(&*q));

    let result = match *origin {
        AbsoluteUri(ref url) => f(url.query.as_ref()),
        AbsolutePath(ref s) => UrlParser::new().parse_path(&*s)
                                                // FIXME: If this fails to parse,
                                                // then it really shouldn't have
                                                // reached here.
                                               .ok()
                                               .and_then(|(_, query, _)| f(query.as_ref())),
        Star | Authority(..) => None
    };

    Query(result.unwrap_or_else(|| HashMap::new()))
}

#[test]
fn splits_and_parses_an_url() {
    use url::Url;
    let t = |url| {
        let store = parse(&url);
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

    assert_eq!(parse(&Star), Query(HashMap::new()));

    let store = parse(&Authority("host.com".to_string()));
    assert_eq!(store, Query(HashMap::new()));
}
