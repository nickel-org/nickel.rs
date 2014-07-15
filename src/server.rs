use std::io::net::ip::{SocketAddr, Ipv4Addr, Port};

use http;
use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};

use router::Router;
use middleware::MiddlewareStack;
use request;
use response;

#[deriving(Clone)]
pub struct Server {
    router: Router,
    middleware_stack: MiddlewareStack,
    port: Port
}

impl http::server::Server for Server {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.port } }
    }

    fn handle_request(&self, req: Request, res: &mut ResponseWriter) {

        let nickel_req = &mut request::Request::from_internal(&req);
        let nickel_res = &mut response::Response::from_internal(res);

        self.middleware_stack.invoke(nickel_req, nickel_res);

        match &req.request_uri {
            &AbsolutePath(ref url) => {
                match self.router.match_route(req.method.clone(), url.clone()) {
                    Some(route_result) => { 
                        nickel_req.params = route_result.params.clone();
                        (route_result.route.handler)(nickel_req, nickel_res);
                    },
                    None => {}
                }
            },
            // TODO: Return 404
            _ => {}
        }
    }
}

impl Server {
    pub fn new(router: Router, middleware_stack: MiddlewareStack, port: Port) -> Server {
        Server {
            router: router,
            middleware_stack: middleware_stack,
            port: port
        }
    }

    // why do we need this? Is the http::Server.serve_forever method protected in C# terms?
    pub fn serve (self) {
        self.serve_forever();
    }
}
