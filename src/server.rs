use std::io::net::ip::{SocketAddr, Ipv4Addr, Port};

use http;
use http::server::request::{AbsolutePath};
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::content_type::MediaType;

use time;

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

    fn handle_request<'a>(&self, req: &Request, res: &'a mut ResponseWriter) {

        //println!("{:?}", req.request_uri)

        fn set_headers(req: &Request, res: &mut ResponseWriter) {
            res.headers.date = Some(time::now_utc());

            // we don't need to set this https://github.com/Ogeon/rustful/issues/3#issuecomment-44787613
            res.headers.content_length = None;
            res.headers.content_type = Some(MediaType {
                type_: String::from_str("text"),
                subtype: String::from_str("plain"),
                parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
            });
            res.headers.server = Some(String::from_str("Example"));
        }

        match &req.request_uri {
            &AbsolutePath(ref url) => {
                match self.router.match_route(req.method.clone(), url.clone()) {
                    Some(route_result) => { 
                        set_headers(req, res); 

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
            _ => set_headers(req, res)
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
