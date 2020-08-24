use crate::router::RouteResult;
use plugin::{Extensible, Pluggable};
use typemap::{Key, ShareMap, TypeMap};
use hyper::{Body, Request as HyperRequest, StatusCode};
use hyper::body::{self, Bytes};
use hyper::header;
use serde::Deserialize;
use serde_json;
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::urlencoded::{self, Params};

/// A container for all the request data.
pub struct Request<D = ()> {
    ///the original `hyper::server::Request`
    pub origin: HyperRequest<Body>,
    body_taken: bool,

    pub route_result: Option<RouteResult>,

    map: ShareMap,

    data: Arc<D>,

    remote_addr: Option<SocketAddr>,

    raw_body_cache: Option<Bytes>,
}

impl<D> Request<D> {
    pub fn from_internal(req: HyperRequest<Body>,
                         remote_addr: Option<SocketAddr>,
                         data: Arc<D>) -> Request<D> {
        Request {
            origin: req,
            body_taken: false,
            route_result: None,
            map: TypeMap::custom(),
            data: data,
            remote_addr: remote_addr,
            raw_body_cache: None
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

    // (Hopefully) temporary replacements for the Extensible trait. We can't
    // support plugins without Extensible, but access to the ShareMap is used by
    // itself.
    pub fn extensions(&self) -> &ShareMap {
        &self.map
    }

    pub fn extensions_mut(&mut self) -> &mut ShareMap {
        &mut self.map
    }

    /// Take the body from the hyper request. Once taken the body is not longer
    /// available. This method will return `None` in that case.
    ///
    /// For small non-streaming applications, the body access methods
    /// `raw_body`, `string_body`, `json_as`, and `form_body` are probably more
    /// convenient. Small in this case is hardware dependent, but even small
    /// modern servers can handle multi-megabyte bodies.
    ///
    /// `take_body` and the body access method are mutually exclusive. Once one
    /// is called, the other will fail.
    pub fn take_body(&mut self) -> Option<Body> {
        if self.body_taken {
            None
        } else {
            let stub = HyperRequest::new(Body::empty());
            let origin = mem::replace(&mut self.origin, stub);
            let (parts, body) = origin.into_parts();
            mem::replace(&mut self.origin, HyperRequest::from_parts(parts, Body::empty()));
            self.body_taken = true;
            Some(body)
        }
    }
}

// TODO: migration cleanup - Extensible does not support ShareMap, but TypeMap is not Sync+Send
// impl<D> Extensible for Request<D> {
//     fn extensions(&self) -> &ShareMap {
//         &self.map
//     }

//     fn extensions_mut(&mut self) -> &mut ShareMap {
//         &mut self.map
//     }
// }

// impl<D> Pluggable for Request<D> {}

// Various body parsers. These used to live in the body_parser module by
// implementing a trait on the Request struct. Async and traits are currently
// kind of cumbersome. We use async_trait for Middleware, because users of
// nickel need to implement Middleware. Since that is not needed for body
// parsers, we will stick with something that will keep the interface simpler.

impl<D> Request<D> {
    /// Extract the raw body from the request. The body is cached so multiple
    /// middleware may access the body. Note that this may consume a lot of
    /// memory when large objects are uploaded.
    ///
    /// To allow access to the body in different ways, `string_body`, `json_as`
    /// and `form_body` all call this and use the same underlying cache.
    pub async fn raw_body(&mut self) -> Result<&[u8], (StatusCode, String)> {
        if let None = self.raw_body_cache {
            // read and insert into cache
            let body = self.take_body().
                ok_or((StatusCode::INTERNAL_SERVER_ERROR, "body already taken".to_string()))?;
            let bytes = body::to_bytes::<Body>(body).await.
                map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            self.raw_body_cache = Some(bytes);
        }
        // we've garanteed this unwrap is safe above
        Ok(self.raw_body_cache.as_ref().unwrap())
    }

    /// Return the body parsed as a `String`. Returns an error if the body is
    /// not uft8.
    pub async fn string_body(&mut self) -> Result<String, (StatusCode, String)> {
        let bytes = self.raw_body().await?;
        String::from_utf8(bytes.to_vec()).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
    }

    /// Uses serde to deserialze thoe body as json into type `T`.
    pub async fn json_as<'a, T: Deserialize<'a>>(&'a mut self) -> Result<T, (StatusCode, String)> {
        let bytes = self.raw_body().await?;
        serde_json::from_slice::<T>(bytes).
            map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
    }

    /// Extract the form data from the body.
    pub async fn form_body(&mut self) -> Result<Params, (StatusCode, String)> {
        // check content type
        match self.origin.headers().get(header::CONTENT_TYPE).map(|v| v.to_str()) {
            Some(Ok("application/x-www-form-urlencoded")) => {
                let s = self.string_body().await?;
                Ok(urlencoded::parse(&s))
            },
            _ => Err((StatusCode::BAD_REQUEST, "Wrong Content Type".to_string()))
        }
    }
}
