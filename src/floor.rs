use std::io::net::ip::{Port};

use http::server::{Request, ResponseWriter};

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

    /// Register a handler to be used for a specific GET request.
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
    ///
    ///     let text = String::new()
    ///                    .append("This is user: ")
    ///                    .append(request.params.get(&"userid".to_string()).as_slice());
    ///
    ///     response.write(text.as_bytes()); 
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
        self.router.add_route(String::from_str(uri), handler);
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
