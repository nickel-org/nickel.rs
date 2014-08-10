use std::io::net::ip::{Port, IpAddr};

use router::Router;
use middleware::{ MiddlewareStack, Middleware, ErrorHandler };
use server::Server;

use http::method::{ Get, Post, Put, Delete };
use request::Request;
use response::Response;


//pre defined middleware
use static_files_handler::StaticFilesHandler;
use json_body_parser::JsonBodyParser;
use query_string::QueryStringParser;
use default_error_handler::DefaultErrorHandler;

/// Nickel is the application object. It's the surface that
/// holds all public APIs.

#[deriving(Clone)]
pub struct Nickel{
    middleware_stack: MiddlewareStack,
    server: Option<Server>,
    default_router: Option<Router>
}
impl Nickel {

    /// In order to use Nickels API one first has to create an instance.
    ///
    /// # Example
    /// ```rust
    /// let mut server = Nickel::new();
    /// ```
    pub fn new() -> Nickel {
        let mut middleware_stack = MiddlewareStack::new();

        // Hook up the default error handler by default. Users are
        // free to cancel it out from their custom error handler if
        // they don't like the default behaviour.
        middleware_stack.add_error_handler(DefaultErrorHandler);

        Nickel {
            middleware_stack: middleware_stack,
            server: None,
            default_router: None
        }
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
    /// fn logger (req: &Request, res: &mut Response) -> Result<Action, NickelError>{
    ///     println!("logging request: {}", req.origin.request_uri);
    ///     Ok(Continue)
    /// }
    /// ```
    pub fn utilize<T: Middleware>(&mut self, handler: T){
        self.middleware_stack.add_middleware(handler);
    }

    /// Registers a handler to be used for a specific GET request.
    /// Handlers are assigned to paths and paths are allowed to contain
    /// variables and wildcards. A handler added through this API will
    /// be attached to the default router. Consider creating the router
    /// middleware manually for advanced functionality.
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
        self.asure_router();
        self.default_router.get_mut_ref().add_route(Get, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific POST request. 
    /// A handler added through this API will be attached to the default router. 
    /// Consider creating the router middleware manually for advanced functionality.
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
        self.asure_router();
        self.default_router.get_mut_ref().add_route(Post, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// A handler added through this API will be attached to the default router. 
    /// Consider creating the router middleware manually for advanced functionality.
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
        self.asure_router();
        self.default_router.get_mut_ref().add_route(Put, String::from_str(uri), handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// A handler added through this API will be attached to the default router. 
    /// Consider creating the router middleware manually for advanced functionality.
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
        self.asure_router();
        self.default_router.get_mut_ref().add_route(Delete, String::from_str(uri), handler);
    }


    fn asure_router (&mut self) {
        if self.default_router.is_none() {
            self.default_router = Some(Router::new());
        }
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
    ///    match err.kind {
    ///        ErrorWithStatusCode(NotFound) => {
    ///            response.origin.headers.content_type = get_media_type("html");
    ///            response.origin.status = NotFound;
    ///            response.send("<h1>Call the police!<h1>");
    ///            Ok(Halt)
    ///        },
    ///        _ => Ok(Continue)
    ///    }
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

    /// Create a new middleware to serve as a router.
    ///
    ///
    /// # Example
    /// ```rust
    /// let mut server = Nickel::new();
    /// let mut router = Nickel::router();
    /// 
    /// router.get("/foo", foo_handler);
    /// server.utilize(router);
    /// ```
    pub fn router() -> Router {
        Router::new()
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

        // At this point it is safe to clone the router because no routes
        // could be attached after `listen` anyway. That implies however
        // that it is not possible to put middleware *after* the default router.
        // One needs to manually create a router and assembly the middleware stack
        // to do that.
        if self.default_router.is_some() {
            let default_router = self.default_router.get_ref().clone();
            self.utilize(default_router);
        }

        self.server = Some(Server::new(self.middleware_stack, ip, port));
        self.server.unwrap().serve();
    }
}
