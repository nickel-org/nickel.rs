//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

#![crate_id = "floor#0.0.1"]
#![comment = "A expressjs inspired web framework for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate time;
extern crate http;
extern crate collections;
extern crate alloc;
extern crate sync;

use collections::hashmap::HashMap;

use std::io::net::ip::{SocketAddr, Ipv4Addr};
use std::rc::Rc;
use alloc::arc::Arc;
use sync::RWLock;

use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

#[deriving(Clone)]
pub struct Floor{
    //routes: HashMap<String, fn(request: &Request, response: &mut ResponseWriter) -> ()>,
    route_store: RouteStore,
    server: Option<Server>
}

#[deriving(Clone)]
struct RouteStore{
    routes: HashMap<String, fn(request: &Request, response: &mut ResponseWriter) -> ()>,
}

impl RouteStore {
    fn new () -> RouteStore {
        RouteStore {
            routes: HashMap::new()
        }
    }
}

#[deriving(Clone)]
pub struct Server {
    route_store: Arc<RWLock<RouteStore>>
}

impl http::server::Server for Server {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, _r: &Request, w: &mut ResponseWriter) {

        //println!("{:?}", _r.request_uri)

        fn set_headers(_r: &Request, w: &mut ResponseWriter) {
            w.headers.date = Some(time::now_utc());

            // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
            w.headers.content_length = None;
            w.headers.content_type = Some(MediaType {
                type_: String::from_str("text"),
                subtype: String::from_str("plain"),
                parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
            });
            w.headers.server = Some(String::from_str("Example"));
        }

        // match &_r.request_uri {
        //     &AbsolutePath(ref url) => {
        //         match self.app.routes.find(url) {
        //             Some(item) => { 
        //                 set_headers(_r, w); 
        //                 (*item)(_r, w);
        //             },
        //             None => {}
        //         }
        //     },
        //     _ => set_headers(_r, w)
        // }
    }
}

impl Server {
    fn new(route_store: Rc<RouteStore>) -> Server {
        Server {
            route_store: route_store
        }
    }
}

impl Floor {
    pub fn get(&mut self, uri: &str, handler: fn(request: &Request, response: &mut ResponseWriter) -> ()){
        self.route_store.routes.insert(String::from_str(uri), handler);
    }

    pub fn new() -> Floor {
        Floor {
            route_store: RouteStore::new(),
            server: None
        }
    }

    //why do we need this. Is serve_forever like a protected method in C# terms?
    pub fn run(&mut self) -> () {
        self.server = Some(Server::new(Arc::new(RWLock(self.route_store))));
        self.server.unwrap().serve_forever();
    }
}