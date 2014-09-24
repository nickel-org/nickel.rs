use http;
use router::RouteResult;
use anymap::AnyMap;

///A container for all the request data
pub struct Request<'a, 'b: 'a> {
    ///the original `http::server::Request`
    pub origin: &'a http::server::Request,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'b>>,

    pub map: AnyMap
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn from_internal(req: &http::server::Request) -> Request {
        Request {
            origin: req,
            route_result: None,
            map: AnyMap::new()
        }
    }

    pub fn param(&self, key: &str) -> &str {
        self.route_result.as_ref().unwrap().param(key)
    }
}
