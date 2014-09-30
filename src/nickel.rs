use std::io::net::ip::{Port, IpAddr};

use router::{Router, RequestHandler, HttpRouter};
use middleware::{MiddlewareStack, Middleware, ErrorHandler, MiddlewareResult};
use nickel_error::{ NickelError, ErrorWithStatusCode };
use server::Server;

use http::method::{ Method, Get, Post, Put, Delete };
use http::status::NotFound;
use request::Request;
use response::Response;

//pre defined middleware
use json_body_parser::JsonBodyParser;
use query_string::QueryStringParser;
use default_error_handler::DefaultErrorHandler;

/// Nickel is the application object. It's the surface that
/// holds all public APIs.
pub struct Nickel{
    middleware_stack: MiddlewareStack,
    server: Option<Server>,
}

impl Nickel {
    /// Creates an instance of Nickel with default error handling.
    pub fn new() -> Nickel {
        let mut middleware_stack = MiddlewareStack::new();

        // Hook up the default error handler by default. Users are
        // free to cancel it out from their custom error handler if
        // they don't like the default behaviour.
        middleware_stack.add_error_handler(DefaultErrorHandler);

        Nickel {
            middleware_stack: middleware_stack,
            server: None
        }
    }

    /// Registers a middleware handler which will be invoked among other middleware
    /// handlers before each request. Middleware can be stacked and is invoked in the
    /// same order it was registered.
    ///
    /// A middleware handler is nearly identical to a regular route handler with the only
    /// difference that it expects a result of either Action or NickelError.
    /// That is to indicate whether other middleware handlers (if any) further
    /// down the stack should continue or if the middleware invocation should
    /// be stopped after the current handler.
    ///
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, Continue, MiddlewareResult};
    /// fn logger(req: &Request, res: &mut Response) -> MiddlewareResult {
    ///     println!("logging request: {}", req.origin.request_uri);
    ///     Ok(Continue)
    /// }
    ///
    /// let mut server = Nickel::new();
    /// server.utilize(logger);
    /// ```
    pub fn utilize<T: Middleware>(&mut self, handler: T){
        self.middleware_stack.add_middleware(handler);
    }

    /// Registers a handler to be used for a specific GET request.
    /// Handlers are assigned to paths and paths are allowed to contain
    /// variables and wildcards.
    ///
    /// A handler added through this API will
    /// be attached to the default router. Consider creating the router
    /// middleware manually for advanced functionality.
    ///
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response};
    /// let mut server = Nickel::new();
    ///
    /// //  without variables or wildcards
    /// fn bare_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user");
    /// };
    /// server.get("/user", bare_handler);
    ///
    /// // with variables
    /// fn var_handler(request: &Request, response: &mut Response) {
    ///     let text = format!("This is user: {}", request.param("userid"));
    ///     response.send(text.as_slice());
    /// };
    /// server.get("/user/:userid", var_handler);
    ///
    /// // with simple wildcard
    /// fn wild_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 but not /user/extended/list/4711");
    /// };
    /// server.get("/user/*/:userid", wild_handler);
    ///
    /// // with double wildcard
    /// fn very_wild_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 and also /user/extended/list/4711");
    /// };
    /// server.get("/user/**/:userid", very_wild_handler);
    /// ```
    pub fn get(&mut self, uri: &str, handler: RequestHandler){
        self.register_route_with_new_router(Get, uri, handler);
    }

    /// Registers a handler to be used for a specific POST request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/post/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// server.post("/a/post/request", handler);
    /// ```
    pub fn post(&mut self, uri: &str, handler: RequestHandler){
        self.register_route_with_new_router(Post, uri, handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/put/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// server.put("/a/put/request", handler);
    /// ```
    pub fn put(&mut self, uri: &str, handler: RequestHandler){
        self.register_route_with_new_router(Put, uri, handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a DELETE request to /a/delete/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// server.delete("/a/delete/request", handler);
    /// ```
    pub fn delete(&mut self, uri: &str, handler: RequestHandler){
        self.register_route_with_new_router(Delete, uri, handler);
    }

    fn register_route_with_new_router(&mut self, method: Method, uri: &str, handler: fn(request: &Request, response: &mut Response)) {
        let mut router = Router::new();
        router.add_route(method, uri, handler);
        self.utilize(router);
    }

    /// Registers an error handler which will be invoked among other error handler
    /// as soon as any regular handler returned an error
    ///
    /// A error handler is nearly identical to a regular middleware handler with the only
    /// difference that it takes an additional error parameter or type `NickelError.
    ///
    /// # Example
    ///
    /// ```{rust}
    /// # extern crate http;
    /// # extern crate nickel;
    /// # fn main() {
    /// use nickel::{Nickel, Request, Response, Continue, Halt, MiddlewareResult};
    /// use nickel::{NickelError, ErrorWithStatusCode, get_media_type};
    /// use http::status::NotFound;
    ///
    /// fn error_handler(err: &NickelError, req: &Request, response: &mut Response)
    ///                  -> MiddlewareResult {
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
    ///
    /// let mut server = Nickel::new();
    /// server.handle_error(error_handler)
    /// # }
    /// ```
    pub fn handle_error<T: ErrorHandler>(&mut self, handler: T){
        self.middleware_stack.add_error_handler(handler);
    }

    /// Create a new middleware to serve as a router.
    ///
    ///
    /// # Example
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    ///
    /// let mut server = Nickel::new();
    /// let mut router = Nickel::router();
    ///
    /// fn foo_handler(request: &Request, response: &mut Response) {
    ///     response.send("Hi from /foo");
    /// };
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
    /// ```{rust}
    /// # #![feature(phase)]
    /// # #[phase(plugin)] extern crate nickel_macros;
    /// # extern crate nickel;
    /// # extern crate serialize;
    /// # use nickel::{Nickel, Request, Response};
    /// use nickel::JsonBody;
    ///
    /// # fn main() {
    /// #[deriving(Decodable, Encodable)]
    /// struct Person {
    ///     first_name: String,
    ///     last_name:  String,
    /// }
    ///
    /// let router = router! {
    ///     post "/a/post/request" => |request, response| {
    ///         let person = request.json_as::<Person>().unwrap();
    ///         let text = format!("Hello {} {}", person.first_name, person.last_name);
    ///         response.send(text);
    ///     }
    /// };
    ///
    /// let mut server = Nickel::new();
    /// // It is currently a requirement that the json_body_parser middleware
    /// // is added before any routes that require it.
    /// server.utilize(Nickel::json_body_parser());
    /// server.utilize(router);
    /// # }
    /// ```
    pub fn json_body_parser() -> JsonBodyParser {
        JsonBodyParser
    }

    /// Create a new middleware to parse the query string.
    ///
    ///
    /// # Example
    /// ```{rust}
    /// # #![feature(phase)]
    /// # #[phase(plugin)] extern crate nickel_macros;
    /// # extern crate nickel;
    /// # use nickel::{Nickel, Request, Response};
    /// use nickel::QueryString;
    /// # fn main() {
    /// let router = router! {
    ///     get "/a/get/request" => |request, response| {
    ///         let foo = request.query("foo", "this is the default value, if foo is not present!");
    ///         response.send(foo[0].as_slice());
    ///     }
    /// };
    ///
    /// let mut server = Nickel::new();
    /// // It is currently a requirement that the query_string middleware
    /// // is added before any routes that require it.
    /// server.utilize(Nickel::query_string());
    /// server.utilize(router);
    /// # }
    /// ```
    pub fn query_string() -> QueryStringParser {
        QueryStringParser
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Example
    /// ```{rust,ignore}
    /// let mut server = Nickel::new();
    /// server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
    /// ```
    pub fn listen(mut self, ip: IpAddr, port: Port) {
        fn not_found_handler(_: &Request, _: &mut Response) -> MiddlewareResult {
            Err(NickelError::new("File Not Found", ErrorWithStatusCode(NotFound)))
        }

        self.middleware_stack.add_middleware(not_found_handler);
        self.server = Some(Server::new(self.middleware_stack, ip, port));
        self.server.unwrap().serve();
    }
}
