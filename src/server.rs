use std::net::IpAddr;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use hyper::server::{Request, Response, Handler};
use hyper::server::Server as HyperServer;

use middleware::MiddlewareStack;
use request;
use response;

pub struct Server {
    middleware_stack: MiddlewareStack,
    templates: response::TemplateCache
}

impl Handler for Arc<Server> {
    fn handle<'a>(&'a self, req: Request<'a>, res: Response<'a>) {
        let nickel_req = request::Request::from_internal(req);
        let nickel_res = response::Response::from_internal(res, &self.templates);

        self.middleware_stack.invoke(nickel_req, nickel_res);
    }
}

impl Server {
    pub fn new(middleware_stack: MiddlewareStack) -> Server {
        Server {
            middleware_stack: middleware_stack,
            templates: RwLock::new(HashMap::new())
        }
    }

    // why do we need this? Is the http::Server.serve_forever method protected in C# terms?
    pub fn serve(self, ip: IpAddr, port: u16) {
        let arc = Arc::new(self);
        let server = HyperServer::http(arc);
        let _ = server.listen(ip, port);
    }
}

