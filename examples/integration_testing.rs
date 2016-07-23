// This attribute being conditional is an implementation detail of the nickel
// test setup for testing examples. Usually, it should just require `#[macro_use]`.
#[cfg_attr(not(test), macro_use)]
extern crate nickel;
extern crate hyper;

#[cfg(not(feature = "with-serde"))]
extern crate rustc_serialize;
#[cfg(feature = "with-serde")]
extern crate serde_json;
#[cfg(feature = "with-serde")]
extern crate serde;

use nickel::{Nickel, ListeningServer, HttpRouter, JsonBody};
use nickel::status::StatusCode;

#[cfg(not(feature = "with-serde"))]
mod json {
    use super::rustc_serialize::json::Json;

    pub fn from_str(json: &str) -> Json {
        Json::from_str(json).unwrap()
    }

    pub trait GetKey {
        fn get<'a>(&'a self, key: &str) -> &'a Json;
    }

    impl GetKey for Json {
        fn get<'a>(&'a self, key: &str) -> &'a Json {
            &self[key]
        }
    }
}
#[cfg(feature = "with-serde")]
mod json {
    use serde_json;
    use serde_json::Value;

    pub fn from_str(json: &str) -> Value {
        serde_json::from_str(json).unwrap()
    }

    pub trait GetKey {
        fn get<'a>(&'a self, key: &str) -> &'a Value;
    }

    impl GetKey for Value {
        fn get<'a>(&'a self, key: &str) -> &'a Value {
            self.find(key).unwrap()
        }
    }
}

use std::error::Error as StdError;
use std::{env, str};
use std::sync::atomic::{self, AtomicUsize};

// Example trait to allow custom databases to be used within the integration tests
// to test different behaviours without touching a real database.
trait Database : Send + Sync + 'static {
    fn get_users(&self) -> Vec<String>;
}

impl Database for Vec<String> {
    fn get_users(&self) -> Vec<String> {
        self.clone()
    }
}

struct ServerData {
    hits: AtomicUsize,

    // FIXME: The `middleware` macro typehinting doesn't support hinting with
    // typeparams, i.e. the hint can't be `< ServerData<T> >` where T is a
    // typeparam from the enclosing function, e.g. `start_server`.
    //
    // To counter this limitation, we've boxed the trait so that specifying the
    // typeparam is unnecessary.
    database: Box<Database>
}

impl ServerData {
    fn hitcount(&self) -> usize {
        self.hits.load(atomic::Ordering::Relaxed)
    }

    fn log_hit(&self) -> usize {
        self.hits.fetch_add(1, atomic::Ordering::Relaxed)
    }

    fn get_users(&self) -> Vec<String> {
        self.database.get_users()
    }
}

#[cfg(not(feature = "with-serde"))]
mod data {
    #[derive(RustcEncodable, RustcDecodable)]
    pub struct Data { pub name: String, pub age: Option<u32> }
}

#[cfg(feature = "with-serde")]
mod data {
    use serde;
    
    pub struct Data { pub name: String, pub age: Option<u32> }
    impl serde::Serialize for Data {
        fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer
        {
            serializer.serialize_struct("Data", DataMapVisitor {
                value: self,
                state: 0,
            })
        }
    }

    struct DataMapVisitor<'a> {
        value: &'a Data,
        state: u8,
    }

    impl<'a> serde::ser::MapVisitor for DataMapVisitor<'a> {
        fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
            where S: serde::Serializer
        {
            match self.state {
                0 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("name", &self.value.name))))
                }
                1 => {
                    self.state += 1;
                    Ok(Some(try!(serializer.serialize_struct_elt("age", &self.value.age))))
                }
                _ => {
                    Ok(None)
                }
            }
        }
    }

    enum DataField {
        Name,
        Age,
    }

    impl serde::Deserialize for DataField {
        fn deserialize<D>(deserializer: &mut D) -> Result<DataField, D::Error>
            where D: serde::de::Deserializer
        {
            struct DataFieldVisitor;

            impl serde::de::Visitor for DataFieldVisitor {
                type Value = DataField;

                fn visit_str<E>(&mut self, value: &str) -> Result<DataField, E>
                    where E: serde::de::Error
                {
                    match value {
                        "name" => Ok(DataField::Name),
                        "age" => Ok(DataField::Age),
                        _ => Err(serde::de::Error::custom("expected name or age")),
                    }
                }
            }

            deserializer.deserialize(DataFieldVisitor)
        }
    }

    impl serde::Deserialize for Data {
        fn deserialize<D>(deserializer: &mut D) -> Result<Data, D::Error>
            where D: serde::de::Deserializer
        {
            static FIELDS: &'static [&'static str] = &["name", "age"];
            deserializer.deserialize_struct("Data", FIELDS, DataVisitor)
        }
    }

    struct DataVisitor;

    impl serde::de::Visitor for DataVisitor {
        type Value = Data;

        fn visit_map<V>(&mut self, mut visitor: V) -> Result<Data, V::Error>
            where V: serde::de::MapVisitor
        {
            let mut name = None;
            let mut age = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(DataField::Name) => { name = Some(try!(visitor.visit_value())); }
                    Some(DataField::Age) => { age = Some(try!(visitor.visit_value())); }
                    None => { break; }
                }
            }

            let name = match name {
                Some(name) => name,
                None => try!(visitor.missing_field("name")),
            };

            let age = match age {
                Some(age) => age,
                None => try!(visitor.missing_field("age")),
            };

            try!(visitor.end());

            Ok(Data{ name: name, age: age })
        }
    }
}

use self::data::Data;

fn main() {
    let port = env::var("PORT").map(|s| s.parse().unwrap()).unwrap_or(3000);
    let address = &format!("0.0.0.0:{}", port);
    let database = vec![];

    start_server(address, database).unwrap();
}

fn start_server<D>(address: &str, database: D) -> Result<ListeningServer, Box<StdError>>
where D: Database {
    let server_data = ServerData {
        hits: AtomicUsize::new(0),
        database: Box::new(database),
    };

    let mut server = Nickel::with_data(server_data);

    // Track all hits to the server
    server.utilize(middleware! { |_req, res| <ServerData>
        res.data().log_hit();
        return res.next_middleware()
    });

    // Core server
    server.get("/", middleware!( "Hello World" ));

    server.get("/users", middleware! { |_, res| <ServerData>
        let users = res.data().get_users();

        json::from_str(&format!(r#"{{ "users": {:?} }}"#, users))
    });

    // Json example
    server.post("/", middleware! { |req, res|
        let client = try_with!(res, req.json_as::<Data>().map_err(|e| (StatusCode::BadRequest, e)));

        match client.age {
            Some(age) => {
                json::from_str(&format!(
                    r#"{{ "message": "Hello {}, your age is {}" }}"#,
                    client.name,
                    age
                ))
            }
            None => {
                json::from_str(&format!(
                    r#"{{ "message": "Hello {}, I don't know your age" }}"#,
                    client.name
                ))
            }
        }
    });

    // Get the hitcount
    server.get("/hits", middleware!{ |_req, res| <ServerData>
        res.data().hitcount().to_string()
    });

    server.listen(address)
}

#[cfg(test)]
mod tests {
    use super::json;
    use super::json::GetKey;
    use self::support::{Body, Server, STATIC_SERVER, get, post};

    use hyper::header;
    use nickel::status::StatusCode;

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

        let json = json::from_str(&response.body());

        assert_eq!(json.get("message").as_string(), Some("Hello Rust, your age is 1"));
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            response.headers.get::<header::ContentType>(),
            Some(&header::ContentType::json())
        );
    }

    #[test]
    fn accepts_json_with_missing_fields() {
        let mut response = post("/", r#"{ "name": "Rust" }"#);

        let json = json::from_str(&response.body());

        assert_eq!(json.get("message").as_string(), Some("Hello Rust, I don't know your age"));
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
            let server = super::start_server("127.0.0.1:0", vec![]).unwrap();
            Server::new(server)
        };

        assert_eq!(get_hits_after_delay(&test_local_server), 1);
    }

    #[test]
    fn has_no_users_by_default() {
        let mut response = get("/users");

        let json = json::from_str(&response.body());

        assert_eq!(json.get("users").as_array().unwrap().len(), 0);
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            response.headers.get::<header::ContentType>(),
            Some(&header::ContentType::json())
        );
    }

    #[test]
    fn non_shared_server_with_different_database() {
        let server = {
            let bots = vec!["bors".into(), "homu".into(), "highfive".into()];
            let server = super::start_server("127.0.0.1:0", bots).unwrap();
            Server::new(server)
        };

        let mut response = server.get("/users");

        let json = json::from_str(&response.body());

        assert_eq!(json.get("users").as_array().unwrap().len(), 3);
        assert_eq!(response.status, StatusCode::Ok);
        assert_eq!(
            response.headers.get::<header::ContentType>(),
            Some(&header::ContentType::json())
        );
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
                let server = super::super::start_server("127.0.0.1:0", vec![]).unwrap();
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
