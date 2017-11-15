use hyper::{self, Method, Uri, HttpVersion, Body};
use plugin::{Extensible, Pluggable};
use router::RouteResult;
use std::mem;
use std::net::SocketAddr;
use typemap::TypeMap;

// As of hyper 0.11.6, the only way to access the request body is to
// take ownership of it, which has the side effect of dropping the
// entire request struct, rendering all the other fields
// unavailable. We deconstruct it and reproduce it here so we can hold
// onto the request metadata after the body has beeen taken.

/// A struct to hold a deconstructed hyper::Request
pub struct HyperRequest {
    method: Method,
    uri: Uri,
    version: HttpVersion,
    headers: hyper::header::Headers,
    body: Option<Body>,
    remote_addr: Option<SocketAddr>,
}

impl HyperRequest {
    pub fn from_hyper(req: hyper::Request) -> HyperRequest {
        let remote_addr = req.remote_addr();
        let (method, uri, version, headers, body) = req.deconstruct();
        HyperRequest{method: method,
                     uri: uri,
                     version: version,
                     headers: headers,
                     body: Some(body),
                     remote_addr: remote_addr}
    }

    pub fn method(&self) -> &Method { &self.method }
    pub fn uri(&self) -> &Uri { &self.uri }
    pub fn path(&self) -> &str { self.uri.path() }
    pub fn version(&self) -> &HttpVersion { &self.version }
    pub fn headers(&self) -> &hyper::header::Headers { &self.headers }
    pub fn remote_addr(&self) -> Option<SocketAddr> { self.remote_addr }

    /// This function releases ownership of the request body to the
    /// caller. If it returns None, another middleware has already
    /// consumed the body.
    pub fn take_body(&mut self) -> Option<Body> {
        let body = mem::replace(&mut self.body, None);
        body
    }

    pub(crate) fn swap_uri(&mut self, new_uri: Uri) -> Uri {
        mem::replace(&mut self.uri, new_uri)
    }
}

/// A container for all the request data.
///
/// The lifetime `'mw` represents the lifetime of various bits of
/// middleware state within nickel. It can vary and get shorter.
///
/// The lifetime `'server` represents the lifetime of data internal to
/// the server. It is fixed and longer than `'mw`.
pub struct Request<'mw, D: 'mw = ()> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'mw, D>>,

    map: TypeMap,

    data: &'mw D,
}

impl<'mw, D> Request<'mw, D> {
    pub fn from_internal(req: hyper::Request,
                         data: &'mw D) -> Request<'mw, D> {
        Request {
            origin: HyperRequest::from_hyper(req),
            route_result: None,
            map: TypeMap::new(),
            data: data
        }
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_result.as_ref().unwrap().param(key)
    }

    pub fn path_without_query(&self) -> &str {
        self.origin.path()
    }

    pub fn server_data(&self) -> &'mw D {
        &self.data
    }
}

impl<'mw, D> Extensible for Request<'mw, D> {
    fn extensions(&self) -> &TypeMap {
        &self.map
    }

    fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.map
    }
}

impl<'mw, D> Pluggable for Request<'mw, D> {}
