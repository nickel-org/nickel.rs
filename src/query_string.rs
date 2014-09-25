use std::collections::hashmap::HashMap;
use request::Request;
use middleware::{Action, Continue, Middleware};
use request;
use response;
use urlencoded;
use nickel_error::NickelError;
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
    fn invoke (&self, req: &mut request::Request, _res: &mut response::Response) -> Result<Action, NickelError> {
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
        self.map.find::<QueryStore>()
            .and_then(| store | {
                match store.find_copy(&key.to_string()) {
                    Some(result) => Some(result),
                    _ => Some(vec![default.to_string().clone()])
                }
            })
            .unwrap()
    }
}

#[test]
fn splits_and_parses_an_url() {
    use url::Url;

    let raw = "http://www.foo.bar/query/test?foo=bar&message=hello&message=world";
    let url = AbsoluteUri(Url::parse(raw).unwrap());
    let store = QueryStringParser::parse(&url);

    assert!(store["foo".to_string()].len() == 1);
    assert!(store["foo".to_string()].contains(&"bar".to_string()));
    assert!(store["message".to_string()].len() == 2);
    assert!(store["message".to_string()].contains(&"hello".to_string()));
    assert!(store["message".to_string()].contains(&"world".to_string()));

    let raw = "/query/test?foo=bar&message=hello&message=world";
    let url = AbsolutePath(raw.to_string());
    let store = QueryStringParser::parse(&url);

    assert!(store["foo".to_string()].len() == 1);
    assert!(store["foo".to_string()].contains(&"bar".to_string()));
    assert!(store["message".to_string()].len() == 2);
    assert!(store["message".to_string()].contains(&"hello".to_string()));
    assert!(store["message".to_string()].contains(&"world".to_string()));

    let url = Star;
    let store = QueryStringParser::parse(&url);

    assert!(store == HashMap::new());

    let url = Authority("host.com".to_string());
    let store = QueryStringParser::parse(&url);

    assert!(store == HashMap::new());
}
