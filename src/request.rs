use router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::server::Request as HyperRequest;
use hyper::uri::RequestUri::AbsolutePath;

//FIXME: Choose better lifetime names
///A container for all the request data
pub struct Request<'a, 'b: 'k, 'k: 'a, D: 'b> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<'a, 'k>,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'b, D>>,

    data: &'a D,

    map: TypeMap,
}

impl<'a, 'b, 'k, D> Request<'a, 'b, 'k, D> {
    pub fn from_internal(req: HyperRequest<'a, 'k>,
                         data: &'a D) -> Request<'a, 'b, 'k, D> {
        Request {
            origin: req,
            route_result: None,
            map: TypeMap::new(),
            data: data
        }
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_result.as_ref().unwrap().param(key)
    }

    pub fn path_without_query(&self) -> Option<&str> {
        match self.origin.uri {
            AbsolutePath(ref path) => Some(path.splitn(2, '?').next().unwrap()),
            _ => None
        }
    }

    pub fn data(&self) -> &D {
        &self.data
    }
}

impl<'a, 'b, 'k, D> Extensible for Request<'a, 'b, 'k, D> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, 'b, 'k, D> Pluggable for Request<'a, 'b, 'k, D> {}
