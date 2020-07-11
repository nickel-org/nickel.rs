use crate::router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::ShareMap;
use hyper::{Body, Request as HyperRequest};
use std::net::SocketAddr;
use std::sync::Arc;

/// A container for all the request data.
///
/// The lifetime `'mw` represents the lifetime of various bits of
/// middleware state within nickel. It can vary and get shorter.
///
/// The lifetime `'server` represents the lifetime of data internal to
/// the server. It is fixed and longer than `'mw`.
pub struct Request<D: = ()> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<Body>,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult>,

    map: SharedMap,

    data: Arc<D>,

    remote_addr: Option<SocketAddr>,
}

impl<D> Request<D> {
    pub fn from_internal(req: HyperRequest<Body>,
                         remote_addr: Option<SocketAddr>,
                         data: Arc<D>) -> Request<D> {
        Request {
            origin: req,
            route_result: None,
            map: SharedMap::new(),
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

    pub fn server_data(&self) -> Arc<D> {
        self.data.clone()
    }

    pub fn remote_addr(&self) -> Option<&SocketAddr> {
        self.remote_addr.as_ref()
    }
}

impl<D> Extensible for Request<D> {
    fn extensions(&self) -> &ShareMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut ShareMap {
        &mut self.map
    }
}

impl<D> Pluggable for Request<D> {}
