use std::io::net::ip::{SocketAddr, IpAddr, Port};

use http;
use http::server::{Config, Server, Request, ResponseWriter};

use middleware::MiddlewareStack;
use request;
use response;

#[deriving(Clone)]
pub struct Server {
    middleware_stack: MiddlewareStack,
    ip: IpAddr,
    port: Port
}

impl http::server::Server for Server {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: self.ip, port: self.port } }
    }

    fn handle_request(&self, req: Request, res: &mut ResponseWriter) {

        let nickel_req = &mut request::Request::from_internal(&req);
        let nickel_res = &mut response::Response::from_internal(res);

        self.middleware_stack.invoke(nickel_req, nickel_res);
    }
}

impl Server {
    pub fn new(middleware_stack: MiddlewareStack, ip: IpAddr, port: Port) -> Server {
        Server {
            middleware_stack: middleware_stack,
            ip: ip,
            port: port
        }
    }

    // why do we need this? Is the http::Server.serve_forever method protected in C# terms?
    pub fn serve (self) {
        self.serve_forever();
    }
}
