use std::collections::HashMap;
use request::Request;
use middleware::{Continue, Middleware, MiddlewareResult};
use request;
use response;
use urlencoded;
use http::server::request::{RequestUri, Star, AbsoluteUri, AbsolutePath, Authority};
use url::UrlParser;

type QueryStore = HashMap<String, Vec<String>>;

#[deriving(Clone)]
pub struct QueryStringParser;

impl QueryStringParser {
    fn parse(origin: &RequestUri) -> QueryStore {
        match *origin {
            AbsoluteUri(ref url) => {
                for query in url.query.iter() {
                    return urlencoded::parse(query.as_slice())
                }
            },
            AbsolutePath(ref s) => {
                match UrlParser::new().parse_path(s.as_slice()) {
                    Ok((_, Some(query), _)) => {
                        return urlencoded::parse(query.as_slice())
                    }
                    Ok(..) => {}
                    // FIXME: If this fails to parse, then it really shouldn't
                    // have reached here.
                    Err(..) => {}
                }
            },
            Star | Authority(..) => {}
        }

        HashMap::new()
    }
}

impl Middleware for QueryStringParser {
    fn invoke(&self, req: &mut request::Request, _: &mut response::Response)
                -> MiddlewareResult {
        let parsed = QueryStringParser::parse(&req.origin.request_uri);
        req.map.insert(parsed);
        Ok(Continue)
    }
}

pub trait QueryString {
    fn query(&self, key: &str, default: &str) -> Vec<String>;
}

impl<'a, 'b> QueryString for request::Request<'a, 'b> {
    fn query(&self, key: &str, default: &str) -> Vec<String> {
        self.map.find::<QueryStore>().and_then(| store | {
            match store.find_copy(&key.to_string()) {
                Some(result) => Some(result),
                _ => Some(vec![default.to_string().clone()])
            }
        }).expect("QueryStore not available. Ensure the middleware \
                  is added before the route that depends on it.")
    }
}

#[test]
fn splits_and_parses_an_url() {
    use url::Url;
    let t = |url|{
        let store = QueryStringParser::parse(&url);
        assert_eq!(store["foo".to_string()], vec!["bar".to_string()]);
        assert_eq!(store["message".to_string()],
                        vec!["hello".to_string(), "world".to_string()]);
    };

    let raw = "http://www.foo.bar/query/test?foo=bar&message=hello&message=world";
    t(AbsoluteUri(Url::parse(raw).unwrap()));

    t(AbsolutePath("/query/test?foo=bar&message=hello&message=world".to_string()));

    assert_eq!(QueryStringParser::parse(&Star), HashMap::new());

    let store = QueryStringParser::parse(&Authority("host.com".to_string()));
    assert_eq!(store, HashMap::new());
}
