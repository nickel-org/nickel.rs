#[macro_use]
extern crate nickel;
extern crate hyper;
extern crate rustc_serialize;

use nickel::{Nickel, ListeningServer, HttpRouter, JsonBody};
use nickel::status::StatusCode;

use rustc_serialize::json::Json;

use std::error::Error as StdError;
use std::{env, str};
use std::sync::atomic::{self, AtomicUsize};

fn main() {
    let port = env::var("PORT").map(|s| s.parse().unwrap()).unwrap_or(3000);

    start_server(&format!("0.0.0.0:{}", port)).unwrap();
}

fn start_server(address: &str) -> Result<ListeningServer, Box<StdError>> {
    let hits = AtomicUsize::new(0);
    let mut server = Nickel::with_data(hits);

    // Track all hits to the server
    server.utilize(middleware! { |_req, res| <AtomicUsize>
        let _hits = res.data().fetch_add(1, atomic::Ordering::Relaxed);
        return res.next_middleware()
    });

    // Core server
    server.get("/", middleware!( "Hello World" ));

    // Json example
    server.post("/", middleware! { |req, res|
        #[derive(RustcEncodable, RustcDecodable)]
        struct Data { name: String, age: Option<u32> }

        let client = try_with!(res, req.json_as::<Data>().map_err(|e| (StatusCode::BadRequest, e)));

        match client.age {
            Some(age) => {
                Json::from_str(&format!(
                    r#"{{ "message": "Hello {}, your age is {}" }}"#,
                    client.name,
                    age
                )).unwrap()
            }
            None => {
                Json::from_str(&format!(
                    r#"{{ "message": "Hello {}, I don't know your age" }}"#,
                    client.name
                )).unwrap()
            }
        }
    });

    // Get the hitcount
    server.get("/hits", middleware!{ |_req, res| <AtomicUsize>
        let hits = res.data().load(atomic::Ordering::Relaxed);

        hits.to_string()
    });

    server.listen(address)
}

#[cfg(test)]
mod tests {
    use self::support::{Body, Server, STATIC_SERVER, get, post};

    use hyper::header;
    use nickel::status::StatusCode;
    use rustc_serialize::json::Json;

    use std::{thread, time};

    fn get_hits_after_delay(server: &Server) -> u32 {
        // let other tests hit the server
        thread::sleep(time::Duration::from_secs(1));

        let mut response = server.get("/hits");
        response.body().parse().unwrap()
    }

    #[test]
    fn root_responds_with_hello_world() {
        let mut response = get("/");

        assert_eq!(response.body(), "Hello World");
        assert_eq!(response.status, StatusCode::Ok);
    }

    #[test]
    // FIXME: This will probably fail if tests are run without parallelism
    fn server_is_shared_with_other_tests() {
        assert!(get_hits_after_delay(&STATIC_SERVER) > 1);
    }

    #[test]
    fn root_responds_with_modified_json() {
        let mut response = post("/", r#"{ "name": "Rust", "age": 1 }"#);

        let json = Json::from_str(&response.body()).unwrap();

        assert_eq!(json["message"].as_string(), Some("Hello Rust, your age is 1"));
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            response.headers.get::<header::ContentType>(),
            Some(&header::ContentType::json())
        );
    }

    #[test]
    fn accepts_json_with_missing_fields() {
        let mut response = post("/", r#"{ "name": "Rust" }"#);

        let json = Json::from_str(&response.body()).unwrap();

        assert_eq!(json["message"].as_string(), Some("Hello Rust, I don't know your age"));
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            response.headers.get::<header::ContentType>(),
            Some(&header::ContentType::json())
        );
    }

    #[test]
    fn doesnt_accept_bad_inputs() {
        let response = post("/", r#"{ }"#);
        assert_eq!(response.status, StatusCode::BadRequest);
    }

    /// This test has a Server instance all to itself.
    #[test]
    fn non_shared_server() {
        let test_local_server = {
            let server = super::start_server("127.0.0.1:0").unwrap();
            Server::new(server)
        };

        assert_eq!(get_hits_after_delay(&test_local_server), 1);
    }

    mod support {
        use hyper::client::{Client, Response as HyperResponse};
        use nickel::ListeningServer;

        use std::net::SocketAddr;

        pub trait Body {
            fn body(self) -> String;
        }

        impl<'a> Body for &'a mut HyperResponse {
            fn body(self) -> String {
                use std::io::Read;
                let mut body = String::new();
                self.read_to_string(&mut body).expect("Failed to read body of Response");
                println!("Read body: {}", body);
                body
            }
        }

        /// An example wrapper type to make testing more readable
        pub struct Server(SocketAddr);
        impl Server {
            pub fn new(server: ListeningServer) -> Server {
                let wrapped = Server(server.socket());

                // detaching is important otherwise it would block the test threads.
                server.detach();

                wrapped
            }

            pub fn get(&self, path: &str) -> HyperResponse {
                let url = self.url_for(path);
                Client::new().get(&url).send().unwrap()
            }

            pub fn post(&self, path: &str, body: &str) -> HyperResponse {
                let url = self.url_for(path);
                Client::new().post(&url).body(body).send().unwrap()
            }

            pub fn url_for(&self, path: &str) -> String {
                format!("http://{}{}", self.0, path)
            }
        }

        lazy_static! {
            /// This is a shared instance of the server between all the tests
            pub static ref STATIC_SERVER: Server = {
                let server = super::super::start_server("127.0.0.1:0").unwrap();
                Server::new(server)
            };
        }

        /// Example of a free function version of `get` which uses the shared server
        pub fn get(path: &str) -> HyperResponse {
            STATIC_SERVER.get(path)
        }

        /// Example of a free function version of `post` which uses the shared server
        pub fn post(path: &str, body: &str) -> HyperResponse {
            STATIC_SERVER.post(path, body)
        }
    }
}
