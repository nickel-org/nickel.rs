use http::server::{Request, ResponseWriter};
use collections::hashmap::HashMap;


#[deriving(Clone)]
pub struct RouteStore{
    pub routes: HashMap<String, fn(request: &Request, response: &mut ResponseWriter)>,
}

impl RouteStore {
    pub fn new () -> RouteStore {
        RouteStore {
            routes: HashMap::new()
        }
    }
}