use std::collections::hashmap::HashMap;
use http;

///A container for all the request data
pub struct Request<'a> {
    ///the original `http::server::Request`
    pub origin: &'a http::server::Request,
    ///a `HashMap<String, String>` holding all params with names and values
    pub params: HashMap<String, String>
}