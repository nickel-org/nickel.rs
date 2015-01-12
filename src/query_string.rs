use std::borrow::Cow;
use std::collections::HashMap;
use request::Request;
use urlencoded;
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::{Star, AbsoluteUri, AbsolutePath, Authority};
use url::UrlParser;
use plugin::{Plugin, Pluggable};
use typemap::Key;

type QueryStore = HashMap<String, Vec<String>>;

// Plugin boilerplate
struct QueryStringParser;
impl Key for QueryStringParser { type Value = QueryStore; }

impl<'a, 'b> Plugin<Request<'a, 'b>> for QueryStringParser {
    type Error = ();

    fn eval(req: &mut Request) -> Result<QueryStore, ()> {
        Ok(parse(&req.origin.uri))
    }
}

pub trait QueryString {
    fn query(&mut self, key: &str, default: &str) -> Cow<[String]>;
}

impl<'a, 'b> QueryString for Request<'a, 'b> {
    fn query(&mut self, key: &str, default: &str) -> Cow<[String]> {
        let store = self.get_ref::<QueryStringParser>()
                        .ok()
                        .expect("Bug: QueryStringParser returned None");

        match store.get(key) {
            Some(result) => Cow::Borrowed(result),
            _ => Cow::Owned(vec![default.to_string()])
        }
    }
}

fn parse(origin: &RequestUri) -> QueryStore {
    let f = |&: query: Option<&String>| query.map(|q| urlencoded::parse(&*q));

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

    result.unwrap_or_else(|| HashMap::new())
}

#[test]
fn splits_and_parses_an_url() {
    use url::Url;
    let t = |&: url|{
        let store = parse(&url);
        assert_eq!(store["foo".to_string()], vec!["bar".to_string()]);
        assert_eq!(store["message".to_string()],
                        vec!["hello".to_string(), "world".to_string()]);
    };

    let raw = "http://www.foo.bar/query/test?foo=bar&message=hello&message=world";
    t(AbsoluteUri(Url::parse(raw).unwrap()));

    t(AbsolutePath("/query/test?foo=bar&message=hello&message=world".to_string()));

    assert_eq!(parse(&Star), HashMap::new());

    let store = parse(&Authority("host.com".to_string()));
    assert_eq!(store, HashMap::new());
}
