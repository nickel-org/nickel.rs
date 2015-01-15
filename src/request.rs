use router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::server::Request as HyperRequest;

///A container for all the request data
pub struct Request<'a, 'b: 'a> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<'a>,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'b>>,

    map: TypeMap
}

impl<'a, 'b> Request<'a, 'b> {
    pub fn from_internal(req: HyperRequest<'a>) -> Request<'a, 'b> {
        Request {
            origin: req,
            route_result: None,
            map: TypeMap::new()
        }
    }

    pub fn param(&self, key: &str) -> &str {
        self.route_result.as_ref().unwrap().param(key)
    }
}

impl<'a, 'b> Extensible for Request<'a, 'b> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, 'b> Pluggable for Request<'a, 'b> {}
