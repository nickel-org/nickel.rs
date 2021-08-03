// This attribute being conditional is an implementation detail of the nickel
// test setup for testing examples. Usually, it should just require `#[macro_use]`.
#[cfg_attr(not(test), macro_use)]
extern crate nickel;

use serde_json;
use async_trait::async_trait;
use nickel::{Nickel, HttpRouter, Request, Response, Middleware, MiddlewareResult};
use serde_derive::{Serialize, Deserialize};
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
    database: Box<dyn Database>
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

struct JsonPost;

#[async_trait]
impl Middleware<ServerData> for JsonPost {
    async fn invoke(&self, req: &mut Request<ServerData>, res: Response<ServerData>) -> MiddlewareResult<ServerData> {
        #[derive(Serialize, Deserialize)]
        struct Data { name: String, age: Option<u32> }

        let client = try_with!(res, req.json_as::<Data>().await);

        let msg = match client.age {
            Some(age) => {
                serde_json::from_str::<serde_json::Value>(&format!(
                    r#"{{ "message": "Hello {}, your age is {}" }}"#,
                    client.name,
                    age
                )).unwrap()
            }
            None => {
                serde_json::from_str::<serde_json::Value>(&format!(
                    r#"{{ "message": "Hello {}, I don't know your age" }}"#,
                    client.name
                )).unwrap()
            }
        };
        res.send(msg)
    }
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").map(|s| s.parse().unwrap()).unwrap_or(0);
    let address = &format!("127.0.0.1:{}", port);
    let database = match env::var("ALT_USERS") {
	Err(_) => vec![],
	Ok(_) => vec!["bors".into(), "homu".into(), "highfive".into()],
    };

    start_server(address, database).await.unwrap();
}

fn log_hit(_req: &mut Request<ServerData>, res: Response<ServerData>) -> MiddlewareResult<ServerData> {
    res.data().log_hit();
    return res.next_middleware()
}

async fn start_server<D>(address: &str, database: D) -> Result<(), Box<dyn StdError>>
where D: Database {
    let server_data = ServerData {
        hits: AtomicUsize::new(0),
        database: Box::new(database),
    };

    let mut server = Nickel::with_data(server_data);

    // Track all hits to the server
    server.utilize(log_hit);

    // Core server
    server.get("/", middleware!( "Hello World" ));

    server.get("/users", middleware! { |_, res| <ServerData>
        let users = res.data().get_users();

        serde_json::from_str::<serde_json::Value>(&format!(r#"{{ "users": {:?} }}"#, users)).unwrap()
    });

    // Json example
    server.post("/", JsonPost);
    // TODO: the middleware macro has not yet been updated to support async    
    // server.post("/", middleware! { |req, res|
    //     #[derive(Serialize, Deserialize)]
    //     struct Data { name: String, age: Option<u32> }

    //     let client = try_with!(res, req.json_as::<Data>().map_err(|e| (StatusCode::BadRequest, e)));

    //     match client.age {
    //         Some(age) => {
    //             serde_json::from_str::<serde_json::Value>(&format!(
    //                 r#"{{ "message": "Hello {}, your age is {}" }}"#,
    //                 client.name,
    //                 age
    //             )).unwrap()
    //         }
    //         None => {
    //             serde_json::from_str::<serde_json::Value>(&format!(
    //                 r#"{{ "message": "Hello {}, I don't know your age" }}"#,
    //                 client.name
    //             )).unwrap()
    //         }
    //     }
    // });

    // Get the hitcount
    server.get("/hits", middleware!{ |_req, res| <ServerData>
        res.data().hitcount().to_string()
    });

    server.listen(address).await
}
