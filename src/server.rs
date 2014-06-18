use std::io::net::ip::{SocketAddr, Ipv4Addr, Port};

use http;
use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};

use router::Router;
use request;
use response;

#[deriving(Clone)]
pub struct Server {
    router: Router,
    port: Port
}

impl http::server::Server for Server {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.port } }
    }

    fn handle_request(&self, req: &Request, res: &mut ResponseWriter) {

        match &req.request_uri {
            &AbsolutePath(ref url) => {
                match self.router.match_route(req.method.clone(), url.clone()) {
                    Some(route_result) => { 

                        let floor_req = request::Request{
                            origin: req,
                            params: route_result.params.clone()
                        };

                        let mut floor_res = response::Response{
                            origin: res
                        };

                        (route_result.route.handler)(floor_req, &mut floor_res);
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
    pub fn new(router: Router, port: Port) -> Server {
        Server {
            router: router,
            port: port
        }
    }

    // why do we need this? Is the http::Server.serve_forever method protected in C# terms?
    pub fn serve (self) {
        self.serve_forever();
    }
}
