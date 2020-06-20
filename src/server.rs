use std::clone::Clone;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use futures_util::future;
use hyper::Result as HttpResult;
use hyper::{Body, Request, Response, StatusCode};
use hyper::server::Server as HyperServer;
use hyper::service::Service;
//use hyper::net::SslServer;

use crate::middleware::MiddlewareStack;
use crate::request;
use crate::response;
use crate::template_cache::{ReloadPolicy, TemplateCache};

pub struct BaseSrv<D> {
    middleware_stack: MiddlewareStack<D>,
    templates: TemplateCache,
    shared_data: D,
}

pub struct Srv<D>(Arc<BaseSrv<D>>);

impl<D> Clone for Srv<D> {
    fn clone(&self) -> Srv<D> {
        Srv(self.0.clone())
    }
}

impl <D: Sync + Send + 'static> Service<Request<Body>> for Srv<D> {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // Creating an empty response, defaulting to 404. We unwrap because this code shouldn't be able to fail.
        //let res = Response::builder().status(StatusCode::NOT_FOUND).body(()).unwrap();
        //let nickel_req = request::Request::from_internal(req,
        //                                                 None, // TODO: get remote address
        //                                                 &self.0.shared_data);
        //let nickel_res = response::Response::from_internal(res,
        //                                                   &self.0.templates,
        //                                                   &self.0.shared_data);
        unimplemented!();
        // self.0.middleware_stack.invoke(nickel_req, nickel_res) // needs to return future::ok(res)
    }
}

pub struct Server<D> {
    base: Srv<D>
}

impl<D: Sync + Send> Server<D> {
    pub fn new(middleware_stack: MiddlewareStack<D>, reload_policy: ReloadPolicy, data: D) -> Server<D> {
        let server_base = BaseSrv {
            middleware_stack: middleware_stack,
            templates: TemplateCache::with_policy(reload_policy),
            shared_data: data
        };
        Server { base: Srv(Arc::new(server_base)) }
    }
}

impl <T, D: Sync + Send> Service<T> for Server<D> {
    type Response = Srv<D>;
    type Error = std::io::Error;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        future::ok(self.base.clone())
    }
}

#[derive(Debug)]
struct ServerError(String);

impl std::error::Error for ServerError { }

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ServerError: {}", self.0)
    }
}

pub async fn serve<A: ToSocketAddrs, D: Sync + Send + 'static>(make_srv: Server<D>,
                                                               addr: A,
                                                               keep_alive_timeout: Option<Duration>,
                                                               thread_count: Option<usize>)
                                                               -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr: SocketAddr = addr.to_socket_addrs()?.next().ok_or(ServerError("bad address".to_string()))?;
    let server = HyperServer::bind(&socket_addr).serve(make_srv);

    println!("Listening on http://{}", socket_addr);

    server.await?;

    Ok(())
}
