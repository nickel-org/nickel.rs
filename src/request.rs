use std::collections::hashmap::HashMap;
use http;
use anymap::AnyMap;

///A container for all the request data
pub struct Request<'a> {
    ///the original `http::server::Request`
    pub origin: &'a http::server::Request,
    ///a `HashMap<String, String>` holding all params with names and values
    pub params: HashMap<String, String>,

    pub map: AnyMap
}

impl<'a> Request<'a> {
    pub fn from_internal<'b>(req: &'b http::server::Request) -> Request<'b>{
        Request {
            origin: req,
            params: HashMap::new(),
            map: AnyMap::new()
        }
    }
}