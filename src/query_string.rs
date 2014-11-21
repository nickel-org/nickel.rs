use std::collections::HashMap;
use request::Request;
use middleware::{Continue, Middleware, MiddlewareResult};
use request;
use response;
use urlencoded;
use http::server::request::RequestUri;
use http::server::request::RequestUri::{Star, AbsoluteUri, AbsolutePath, Authority};
use url::UrlParser;
use plugin::{Phantom, PluginFor, GetCached};
use typemap::Assoc;

type QueryStore = HashMap<String, Vec<String>>;

// Plugin boilerplate
struct QueryStringParser;
impl Assoc<QueryStore> for QueryStringParser {}
impl<'a, 'b> PluginFor<Request<'a, 'b>, QueryStore> for QueryStringParser {
    fn eval(req: &mut Request, _: Phantom<QueryStringParser>) -> Option<QueryStore> {
        Some(QueryStringParser::parse(&req.origin.request_uri))
    }
}

pub trait QueryString {
    fn query(&mut self, key: &str, default: &str) -> Vec<String>;
}

impl<'a, 'b> QueryString for Request<'a, 'b> {
    fn query(&mut self, key: &str, default: &str) -> Vec<String> {
        self.get_ref::<QueryStringParser>().and_then(|store| {
            match store.get(key).cloned() {
                Some(result) => Some(result),
                _ => Some(vec![default.to_string().clone()])
            }
        }).expect("Bug: QueryStringParser returned None")
    }
}

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
