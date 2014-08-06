use std::io::net::ip::{Port, IpAddr};

use http::method;

use router::Router;
use middleware::{ MiddlewareStack, Middleware, ErrorHandler };
use server::Server;
use request::Request;
use response::Response;

//pre defined middleware
use static_files_handler::StaticFilesHandler;
use json_body_parser::JsonBodyParser;
use query_string::QueryStringParser;

///Nickel is the application object. It's the surface that
///holds all public APIs.

#[deriving(Clone)]
pub struct Nickel{
    router: Router,
    middleware_stack: MiddlewareStack,
    server: Option<Server>
}
impl Nickel {

    /// In order to use Nickels API one first has to create an instance.
    ///
    /// # Example
    /// ```rust
    /// let mut server = Nickel::new();
    /// ```
    pub fn new() -> Nickel {
        let router = Router::new();
        let middleware_stack = MiddlewareStack::new();
        Nickel {
            router: router,
            middleware_stack: middleware_stack,
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
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches /user");
    /// };
    /// server.get("/user", handler);
    /// ```
    /// # Example with variables
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     let text = format!("This is user: {}", request.params.get(&"userid".to_string()));
    ///     response.send(text.as_slice());
    /// };
    /// server.get("/user/:userid", handler);
    /// ```
    /// # Example with simple wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 but not /user/extended/list/4711");
    /// };
    /// server.get("/user/*/:userid", handler);
    /// ```
    /// # Example with double wildcard
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 and also /user/extended/list/4711");
    /// };
    /// server.get("/user/**/:userid", handler);
    /// ```
    pub fn get(&mut self, uri: &str, handler: fn(request: &Request, response: &mut Response)){
        self.router.add_route(method::Get, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific POST request.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/post/request");
    /// };
    /// server.post("/a/post/request", handler);
    /// ```
    /// Take a look at `get()` for a more detailed description.
    pub fn post(&mut self, uri: &str, handler: fn(request: &Request, response: &mut Response)){
        self.router.add_route(method::Post, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/put/request");
    /// };
    /// server.put("/a/put/request", handler);
    /// ```
    /// Take a look at `get(..)` for a more detailed description.
    pub fn put(&mut self, uri: &str, handler: fn(request: &Request, response: &mut Response)){
        self.router.add_route(method::Put, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn handler (request: Request, response: &mut Response) {
    ///     response.send("This matches a DELETE request to /a/delete/request");
    /// };
    /// server.delete("/a/delete/request", handler);
    /// ```
    /// Take a look at `get(...)` for a more detailed description.
    pub fn delete(&mut self, uri: &str, handler: fn(request: &Request, response: &mut Response)){
        self.router.add_route(method::Delete, String::from_str(uri), handler);
    }

    /// Registers a middleware handler which will be invoked among other middleware
    /// handlers before each request. Middleware can be stacked and is invoked in the
    /// same order it was registered.
    ///
    /// A middleware handler is nearly identical to a regular route handler with the only
    /// difference that it expects a return value of boolean. That is to indicate whether
    /// other middleware handler (if any) further down the stack should continue or if the
    /// middleware invocation should be stopped after the current handler.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn logger (req: &Request, res: &mut Response) -> Action{
    ///     println!("logging request: {}", req.origin.request_uri);
    ///     Continue
    /// }
    /// ```
    pub fn utilize<T: Middleware>(&mut self, handler: T){
        self.middleware_stack.add_middleware(handler);
    }

    /// Registers an error handler which will be invoked among other error handler
    /// as soon as any regular handler returned an error
    ///
    /// A error handler is nearly identical to a regular middleware handler with the only
    /// difference that it takes an additional error parameter or type `NickelError.
    ///
    /// # Example
    ///
    /// ```rust
    /// fn error_handler (err: &NickelError, req: &Request, res: &mut Response) -> Result<Action, NickelError>{
    ///     //handle error
    ///     Ok(Continue)
    /// }
    /// ```
    pub fn handle_error<T: ErrorHandler>(&mut self, handler: T){
        self.middleware_stack.add_error_handler(handler);
    }

    /// Create a new middleware to serve files from within a given root directory.
    /// The file to serve will be determined by combining the requested Url with
    /// the provided root directory.
    ///
    ///
    /// # Example
    /// ```rust
    /// let mut server = Nickel::new();
    /// server.utilize(Nickel::static_files("/path/to/serve/"));
    /// ```
    pub fn static_files(root_path: &str) -> StaticFilesHandler {
        StaticFilesHandler::new(root_path)
    }

    /// Create a new middleware to parse JSON bodies.
    ///
    ///
    /// # Example
    /// ```rust
    ///
    /// #[deriving(Decodable, Encodable)]
    /// struct Person {
    ///     firstname: String,
    ///     lastname:  String,
    /// }
    ///
    /// let mut server = Nickel::new();
    /// server.utilize(Nickel::json_body_parser();
    ///
    /// fn post_handler (request: &Request, response: &mut Response) {
    ///
    ///     let person = request.json_as::<Person>().unwrap();
    ///     let text = format!("Hello {} {}", person.firstname, person.lastname);
    ///     response.send(text.as_slice());
    /// };
    ///
    /// server.post("/a/post/request", post_handler);
    /// ```
    pub fn json_body_parser() -> JsonBodyParser {
        JsonBodyParser
    }

    /// Create a new middleware to parse the query string.
    ///
    ///
    /// # Example
    /// ```rust
    ///
    /// let mut server = Nickel::new();
    /// server.utilize(Nickel::query_string();
    ///
    /// fn get_handler (request: &Request, response: &mut Response) {
    ///     let foo = request.query("foo", "this is the default value, if foo is not present!");
    ///     response.send(foo.as_slice());
    /// };
    ///
    /// server.get("/a/get/request", get_handler);
    /// ```
    pub fn query_string() -> QueryStringParser {
        QueryStringParser
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Example
    /// ```rust
    /// let mut server = Nickel::new();
    /// server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
    /// ```
    pub fn listen(mut self, ip: IpAddr, port: Port) {
        self.server = Some(Server::new(self.router, self.middleware_stack, ip, port));
        self.server.unwrap().serve();
    }
}
