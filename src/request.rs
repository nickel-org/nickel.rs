use collections::hashmap::HashMap;
use http;

pub struct Request<'a> {
    pub origin: &'a http::server::Request,
    pub variables: HashMap<String, String>
}