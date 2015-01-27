use std::borrow::Cow;
use std::collections::HashMap;
use request::Request;
use urlencoded;
use http::server::request::RequestUri;
use http::server::request::RequestUri::{Star, AbsoluteUri, AbsolutePath, Authority};
use url::UrlParser;
use plugin::{Phantom, Plugin, Pluggable};
use typemap::Key;

type QueryStore = HashMap<String, Vec<String>>;

// Plugin boilerplate
struct QueryStringParser;
impl Key for QueryStringParser { type Value = QueryStore; }

impl<'a, 'b> Plugin<Request<'a, 'b>> for QueryStringParser {
    fn eval(req: &mut Request, _: Phantom<QueryStringParser>) -> Option<QueryStore> {
        Some(parse(&req.origin.request_uri))
    }
}

pub trait QueryString {
    // FIXME: This would probably be better to return Cow<Vec<String>, [String]>
    // but ToOwned isn't implemented for that combination yet.
    fn query(&mut self, key: &str, default: &str) -> Cow<Vec<String>, Vec<String>>;
}

impl<'a, 'b> QueryString for Request<'a, 'b> {
    fn query(&mut self, key: &str, default: &str) -> Cow<Vec<String>, Vec<String>> {
        let store = self.get_ref::<QueryStringParser>()
                        .expect("Bug: QueryStringParser returned None");

        match store.get(key) {
            Some(result) => Cow::Borrowed(result),
            _ => Cow::Owned(vec![default.to_string()])
        }
    }
}

fn parse(origin: &RequestUri) -> QueryStore {
    let f = |&: query: Option<&String>| query.map(|q| urlencoded::parse(&q[]));

    let result = match *origin {
        AbsoluteUri(ref url) => f(url.query.as_ref()),
        AbsolutePath(ref s) => UrlParser::new().parse_path(&s[])
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
