use std::clone::Clone;
use std::convert::Infallible;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;
use hyper::{Body, Request, Response, StatusCode};
use hyper::server::Server as HyperServer;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
//use hyper::net::SslServer;

use crate::middleware::MiddlewareStack;
use crate::request;
use crate::response;
use crate::template_cache::{ReloadPolicy, TemplateCache};

pub struct Server<D: Send + 'static + Sync> {
    middleware_stack: Arc<MiddlewareStack<D>>,
    templates: Arc<TemplateCache>,
    shared_data: Arc<D>,
}

impl<D: Sync + Send + 'static> Server<D> {
    pub fn new(middleware_stack: MiddlewareStack<D>, reload_policy: ReloadPolicy, data: D) -> Server<D> {
        Server {
            middleware_stack: Arc::new(middleware_stack),
            templates: Arc::new(TemplateCache::with_policy(reload_policy)),
            shared_data: Arc::new(data)
        }
    }

    pub async fn serve<A: ToSocketAddrs>(self,
                                         addr: A,
                                         keep_alive_timeout: Option<Duration>, // TODO: migration cleanup - use this
                                         thread_count: Option<usize>) // TODO: migration cleanup - use or remove this
                                         -> Result<(), Box<dyn std::error::Error>> {
        let socket_addr: SocketAddr = addr.to_socket_addrs()?.next().ok_or(ServerError("bad address".to_string()))?;

        let make_svc = make_service_fn(move |socket: &AddrStream| {
            let remote_addr = socket.remote_addr();
            let mw = self.middleware_stack.clone();
            let data = self.shared_data.clone();
            let res_templates = self.templates.clone();
            async move {
                Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                    let mw2 = mw.clone();
                    let req_data2 = data.clone();
                    let res_data2 = data.clone();
                    let res_templates2 = res_templates.clone();
                    async move {
                        let res = Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty()).unwrap();
                        let nickel_req = request::Request::from_internal(req,
                                                                         Some(remote_addr.to_owned()),
                                                                         req_data2);
                        let nickel_res = response::Response::from_internal(res,
                                                                           res_templates2,
                                                                           res_data2);
                        let final_res = mw2.invoke(nickel_req, nickel_res).await;
                        Ok::<_, Infallible>(final_res)
                    }
                }))
            }
        });
        let server = HyperServer::bind(&socket_addr).serve(make_svc);

        println!("Listening on http://{}", socket_addr);
        
        server.await?;
        
        Ok(())
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
