use router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::server::Request as HyperRequest;
use hyper::uri::RequestUri::AbsolutePath;

/// A container for all the request data.
///
/// The lifetime `'mw` represents the lifetime of various bits of
/// middleware state within nickel. It can vary and get shorter.
///
/// The lifetime `'server` represents the lifetime of data internal to
/// the server. It is fixed and longer than `'mw`.
pub struct Request<'mw, 'server: 'mw, D: 'mw = ()> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<'mw, 'server>,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'mw, D>>,

    map: TypeMap,

    data: &'mw D,
}

impl<'mw, 'server, D> Request<'mw, 'server, D> {
    pub fn from_internal(req: HyperRequest<'mw, 'server>,
                         data: &'mw D) -> Request<'mw, 'server, D> {
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

    pub fn server_data(&self) -> &'mw D {
        &self.data
    }
}

impl<'mw, 'server, D> Extensible for Request<'mw, 'server, D> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'mw, 'server, D> Pluggable for Request<'mw, 'server, D> {}
