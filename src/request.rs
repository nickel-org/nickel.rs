use router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::TypeMap;
use hyper::Request as HyperRequest;
use hyper::Uri;
use hyper;
use futures::{Stream, Future};
use std::net::SocketAddr;

#[derive(Debug)]
pub struct RequestOrigin {
    pub method: hyper::Method,
    pub uri: Uri,
    pub http_version: hyper::HttpVersion,
    pub headers: hyper::Headers,
    pub body: Option<Vec<u8>>,
    pub remote_addr: Option<SocketAddr>
}

impl RequestOrigin {
    pub fn from_internal(req: HyperRequest) -> Box<Future<Item = RequestOrigin, Error = hyper::Error>> {

        let ra = req.remote_addr();

        let (method, uri, http_version, headers, body) = req.deconstruct();

        Box::new(body.concat2().map(move |full_body| {

            println!("{:?} {:?}", method, uri);
            println!("body {:?}", full_body);
            RequestOrigin {
                method: method,
                // path: uri.path().to_owned(),
                uri: uri,
                http_version: http_version,
                headers: headers,
                body: Some(full_body.to_vec()),
                remote_addr: ra
            }

        }))
                
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
    pub origin: &'mw RequestOrigin,
    ///a `HashMap<String, String>` holding all params with names and values
    pub route_result: Option<RouteResult<'mw, D>>,

    map: TypeMap,

    data: &'mw D,

    pub path: String
}

impl<'mw, 'server, D> Request<'mw, D> {
    pub fn from_internal(origin: &'mw RequestOrigin,
                         data: &'mw D) -> Request<'mw, D> {
        Request {
            origin: origin,
            route_result: None,
            map: TypeMap::new(),
            data: data,
            path: origin.uri.path().to_owned()
        }
    }

    pub fn body(&self) -> Option<Vec<u8>> {
        self.origin.body.clone()
    }

    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_result.as_ref().unwrap().param(key)
    }

    pub fn path_without_query(&self) -> Option<&str> {
        self.path.splitn(2, '?').next()
    }

    pub fn server_data(&self) -> &'mw D {
        &self.data
    }

    pub fn update_path(&mut self, new_path: String) -> String {
        use std::mem;
        mem::replace(&mut self.path, new_path)
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
