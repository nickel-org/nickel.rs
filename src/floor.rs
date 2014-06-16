use std::io::net::ip::{Port};

use http::server::{Request, ResponseWriter};
use http::method;

use router::Router;
use server::Server;
use request;

///Floor is the application object. It's the surface that 
///holds all public APIs.

#[deriving(Clone)]
pub struct Floor{
    router: Router,
    server: Option<Server>
}
impl Floor {

    /// In order to use Floors API one first has to create an instance.
    ///
    /// # Example
    /// ```rust
    /// let mut server = Floor::new();
    /// ```
    pub fn new() -> Floor {
        let routes = Router::new();
        Floor {
            router: routes,
            server: None,
        }
    }

    /// Registers a handler to be used for a specific GET request.
    /// Handlers are assigned to paths and paths are allowed to contain
    /// variables and wildcards.
    ///
    /// # Example without variables and wildcards
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) { 
    ///     response.write("This matches /user".as_bytes()); 
    /// };
    /// server.get("/user", handler);
    /// ```
    /// # Example with variables
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     let _ = write!(response, "This is user: {}", request.params.get(&"userid".to_string()));
    /// };
    /// server.get("/user/:userid", handler);
    /// ```
    /// # Example with simple wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     response.write("This matches /user/list/4711 but not /user/extended/list/4711".as_bytes());  
    /// };
    /// server.get("/user/*/:userid", handler);
    /// ```
    /// # Example with double wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     response.write("This matches /user/list/4711 and also /user/extended/list/4711".as_bytes());  
    /// };
    /// server.get("/user/**/:userid", handler);
    /// ```
    pub fn get(&mut self, uri: &str, handler: fn(request: request::Request, response: &mut ResponseWriter)){
        self.router.add_route(method::Get, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific POST request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     response.write("This matches a POST request to /a/post/request".as_bytes());  
    /// };
    /// server.post("/a/post/request", handler);
    /// ```
    /// Take a look at `get()` for a more detailed description.
    pub fn post(&mut self, uri: &str, handler: fn(request: request::Request, response: &mut ResponseWriter)){
        self.router.add_route(method::Post, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     response.write("This matches a POST request to /a/put/request".as_bytes());  
    /// };
    /// server.put("/a/put/request", handler);
    /// ```
    /// Take a look at `get(..)` for a more detailed description.
    pub fn put(&mut self, uri: &str, handler: fn(request: request::Request, response: &mut ResponseWriter)){
        self.router.add_route(method::Put, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// 
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut ResponseWriter) {
    ///     response.write("This matches a DELETE request to /a/delete/request".as_bytes());  
    /// };
    /// server.delete("/a/delete/request", handler);
    /// ```
    /// Take a look at `get(...)` for a more detailed description.
    pub fn delete(&mut self, uri: &str, handler: fn(request: request::Request, response: &mut ResponseWriter)){
        self.router.add_route(method::Put, String::from_str(uri), handler);
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Example
    /// ```rust
    /// let mut server = Floor::new();
    /// server.listen(6767);
    /// ```
    pub fn listen(mut self, port: Port) {
        self.server = Some(Server::new(self.router.clone(), port));
        self.server.unwrap().serve();
    }
}
