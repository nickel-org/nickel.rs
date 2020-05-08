use crate::router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::Request as HyperRequest;
use std::net::SocketAddr;

/// A container for all the request data.
///
/// The lifetime `'mw` represents the lifetime of various bits of
/// middleware state within nickel. It can vary and get shorter.
///
/// The lifetime `'server` represents the lifetime of data internal to
/// the server. It is fixed and longer than `'mw`.
pub struct Request<'mw, B, D: 'mw = ()> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<B>,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'mw, D>>,

    map: TypeMap,

    data: &'mw D,

    remote_addr: Option<SocketAddr>,
}

impl<'mw, B, D> Request<'mw, B, D> {
    pub fn from_internal(req: HyperRequest<B>,
                         remote_addr: Option<SocketAddr>,
                         data: &'mw D) -> Request<'mw, B, D> {
        Request {
            origin: req,
            route_result: None,
            map: TypeMap::new(),
            data: data,
            remote_addr: remote_addr
        }
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_result.as_ref().unwrap().param(key)
    }

    pub fn path_without_query(&self) -> &str {
        self.origin.uri().path()
    }

    pub fn server_data(&self) -> &'mw D {
        &self.data
    }

    pub fn remote_addr(&self) -> Option<&SocketAddr> {
        self.remote_addr.as_ref()
    }
}

impl<'mw, B, D> Extensible for Request<'mw, B, D> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'mw, B, D> Pluggable for Request<'mw, B, D> {}
