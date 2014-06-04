//! A very simple HTTP server which responds with the plain text "Hello, World!" to every request.

#![crate_id = "floor#0.0.1"]
#![comment = "A expressjs inspired web framework for Rust"]
#![license = "MIT"]
#![crate_type = "rlib"]

extern crate time;
extern crate http;
extern crate collections;

use collections::hashmap::HashMap;

use std::io::net::ip::{SocketAddr, Ipv4Addr};

use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

#[deriving(Clone)]
pub struct Floor{
    routes: HashMap<String, fn(request: &Request, response: &mut ResponseWriter) -> ()>,
}

impl Server for Floor {
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

        match &_r.request_uri {
            &AbsolutePath(ref url) => {
                match self.routes.find(url) {
                    Some(item) => { 
                        set_headers(_r, w); 
                        (*item)(_r, w);
                    },
                    None => {}
                }
            },
            _ => set_headers(_r, w)
        }
    }


}

impl Floor {
    pub fn get(&mut self, uri: &str, handler: fn(request: &Request, response: &mut ResponseWriter) -> ()){
        self.routes.insert(String::from_str(uri), handler);
    }

    pub fn create_server() -> Floor {
        Floor {
            routes: HashMap::new()
        }
    }

    //why do we need this. Is serve_forever like a protected method in C# terms?
    pub fn run(self) -> () {
        self.serve_forever();
    }
}