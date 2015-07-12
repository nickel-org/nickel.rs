use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::server::Request as HyperRequest;
use hyper::uri::RequestUri::AbsolutePath;

//FIXME: Choose better lifetime names
///A container for all the request data
pub struct Request<'a, 'k: 'a> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<'a, 'k>,

    map: TypeMap,
}

impl<'a, 'k> Request<'a, 'k> {
    pub fn from_internal(req: HyperRequest<'a, 'k>) -> Request<'a, 'k> {
        Request {
            origin: req,
            map: TypeMap::new()
        }
    }

    // pub fn param(&self, key: &str) -> Option<&str> {
    //     self.route_result.as_ref().unwrap().param(key)
    // }

    pub fn path_without_query(&self) -> Option<&str> {
        match self.origin.uri {
            AbsolutePath(ref path) => Some(path.splitn(2, '?').next().unwrap()),
            _ => None
        }
    }
}

impl<'a, 'k> Extensible for Request<'a, 'k> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'a, 'k> Pluggable for Request<'a, 'k> {}
