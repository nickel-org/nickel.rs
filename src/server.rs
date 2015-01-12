use std::old_io::net::ip::{SocketAddr, IpAddr, Port};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use hyper::server::{Request, Response, Handler};
use hyper::server::Server as HyperServer;

use middleware::MiddlewareStack;
use request;
use response;
use mustache;

pub struct Server {
    middleware_stack: MiddlewareStack,
    ip: IpAddr,
    port: Port,
    templates: response::TemplateCache
}

impl Handler for Arc<Server> {
    fn handle<'a>(&'a self, req: Request<'a>, res: Response<'a>) {
        let nickel_req = request::Request::from_internal(&req);
        let nickel_res = response::Response::from_internal(res, &self.templates);

        self.middleware_stack.invoke(nickel_req, nickel_res);
    }
}

impl Server {
    pub fn new(middleware_stack: MiddlewareStack, ip: IpAddr, port: Port) -> Server {
        Server {
            middleware_stack: middleware_stack,
            ip: ip,
            port: port,
            templates: RwLock::new(HashMap::<&'static str, mustache::Template>::new())
        }
    }

    // why do we need this? Is the http::Server.serve_forever method protected in C# terms?
    pub fn serve(self) {
        let socket = HyperServer::http(self.ip, self.port);
        let arc = Arc::new(self);
        socket.listen(arc);
    }
}

