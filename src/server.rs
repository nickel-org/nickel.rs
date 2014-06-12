use std::io::net::ip::{SocketAddr, Ipv4Addr, Port};

use http;
use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use time;

use router::Router;
use request;

#[deriving(Clone)]
pub struct Server {
    router: Router,
    port: Port
}

impl http::server::Server for Server {
    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: self.port } }
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
                match self.router.match_route(url.clone()) {
                    Some(item) => { 
                        set_headers(_r, w); 

                        let req = request::Request{
                            origin: _r,
                            variables: item.variables.clone()
                        };

                        (item.route.handler)(req, w);
                    },
                    None => {}
                }
            },
            _ => set_headers(_r, w)
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
