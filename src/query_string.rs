use std::collections::hashmap::HashMap;
use request::Request;
use middleware::{Action, Continue, Middleware};
use request;
use response;
use urlencoded;
use nickel_error::NickelError;

type QueryStore = HashMap<String, Vec<String>>;

#[deriving(Clone)]
pub struct QueryStringParser;

impl QueryStringParser {
    fn parse_url(url : String) -> QueryStore {
        let elements : Vec<&str> = url.as_slice().split('?').collect();
        let store:QueryStore = if elements.len() == 2 { urlencoded::parse(elements[1]) } else { HashMap::new() };
        store
    }
}

impl Middleware for QueryStringParser {
    fn invoke (&self, req: &mut request::Request, _res: &mut response::Response) -> Result<Action, NickelError> {
        let temp = req.origin.request_uri.to_string();
        req.map.insert(QueryStringParser::parse_url(temp));
        Ok(Continue)
    }
}

pub trait QueryString {
    fn query(&self, key: &str, default: &str) -> Vec<String>;
}

impl<'a> QueryString for request::Request<'a> {
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
    let store = QueryStringParser::parse_url(
      "http://www.foo.bar/query/test?foo=bar&message=hello&message=world".to_string()
    );
    assert!(store.get(&"foo".to_string()).len() == 1);
    assert!(store.get(&"foo".to_string()).contains(&"bar".to_string()));
    assert!(store.get(&"message".to_string()).len() == 2);
    assert!(store.get(&"message".to_string()).contains(&"hello".to_string()));
    assert!(store.get(&"message".to_string()).contains(&"world".to_string()));
}
