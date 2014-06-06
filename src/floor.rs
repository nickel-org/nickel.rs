use std::io::net::ip::{Port};

use http::server::{Request, ResponseWriter};

use routestore::RouteStore;
use server::Server;

#[deriving(Clone)]
pub struct Floor{
    route_store: RouteStore,
    server: Option<Server>
}


impl Floor {
    pub fn get(&mut self, uri: &str, handler: fn(request: &Request, response: &mut ResponseWriter)){
        self.route_store.routes.insert(String::from_str(uri), handler);
    }

    pub fn new() -> Floor {
        let routes = RouteStore::new();
        Floor {
            route_store: routes,
            server: None,
        }
    }

    pub fn listen(mut self, port: Port) {
        self.server = Some(Server::new(self.route_store.clone(), port));
        self.server.unwrap().serve();
    }
}