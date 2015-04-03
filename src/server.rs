use std::net::ToSocketAddrs;
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

// FIXME: Any better coherence solutions?
struct ArcServer(Arc<Server>);

impl Handler for ArcServer {
    fn handle<'a, 'k>(&'a self, req: Request<'a, 'k>, res: Response<'a>) {
        let nickel_req = request::Request::from_internal(req);
        let nickel_res = response::Response::from_internal(res, &self.0.templates);

        self.0.middleware_stack.invoke(nickel_req, nickel_res);
    }
}

impl Server {
    pub fn new(middleware_stack: MiddlewareStack) -> Server {
        Server {
            middleware_stack: middleware_stack,
            templates: RwLock::new(HashMap::new())
        }
    }

    pub fn serve<T: ToSocketAddrs>(self, addr: T) {
        let arc = ArcServer(Arc::new(self));
        let server = HyperServer::http(arc);
        let _ = server.listen(addr);
    }
}

