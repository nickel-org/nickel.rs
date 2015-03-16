use std::net::IpAddr;
use router::{Router, HttpRouter};
use middleware::{MiddlewareStack, Middleware, ErrorHandler};
use server::Server;
use hyper::method::Method;
use hyper::status::StatusCode;

// Re-exports so that we can use nickel_macros within nickel
// as they use the path `nickel::foo` which resolves to this module
// rather than an external crate.
pub use {MiddlewareResult, ResponseFinalizer, Request, Response};

//pre defined middleware
use default_error_handler::DefaultErrorHandler;

/// Nickel is the application object. It's the surface that
/// holds all public APIs.
pub struct Nickel{
    middleware_stack: MiddlewareStack,
}

impl HttpRouter for Nickel {
    fn add_route<H: Middleware>(&mut self, method: Method, uri: &str, handler: H) {
        let mut router = Router::new();
        // FIXME: Inference failure in nightly 22/10/2014
        router.add_route::<H>(method, uri, handler);
        self.utilize(router);
    }
}

impl Nickel {
    /// Creates an instance of Nickel with default error handling.
    pub fn new() -> Nickel {
        let mut middleware_stack = MiddlewareStack::new();

        // Hook up the default error handler by default. Users are
        // free to cancel it out from their custom error handler if
        // they don't like the default behaviour.
        middleware_stack.add_error_handler(DefaultErrorHandler);

        Nickel { middleware_stack: middleware_stack }
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
    /// # Examples
    /// ```{rust}
    /// # extern crate nickel;
    /// # #[macro_use] extern crate nickel_macros;
    /// # fn main() {
    /// use nickel::Nickel;
    /// let mut server = Nickel::new();
    /// server.utilize(middleware! { |req|
    ///     println!("logging request: {:?}", req.origin.uri);
    /// });
    /// # }
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
    /// # Examples
    ///
    /// ```{rust}
    /// # extern crate nickel;
    /// # fn main() {
    /// use std::io::Write;
    /// use nickel::{Nickel, Request, Response, Continue, Halt};
    /// use nickel::{NickelError, Action};
    /// use nickel::status::StatusCode::NotFound;
    ///
    /// fn error_handler(err: &mut NickelError, req: &mut Request) -> Action {
    ///    if let Some(ref mut res) = err.stream {
    ///        if let NotFound = res.status() {
    ///            let _ = res.write_all(b"<h1>Call the police!</h1>");
    ///            return Halt(())
    ///        }
    ///    }
    ///
    ///     Continue(())
    /// }
    ///
    /// let mut server = Nickel::new();
    ///
    /// let ehandler: fn(&mut NickelError, &mut Request) -> Action = error_handler;
    ///
    /// server.handle_error(ehandler)
    /// # }
    /// ```
    pub fn handle_error<T: ErrorHandler>(&mut self, handler: T){
        self.middleware_stack.add_error_handler(handler);
    }

    /// Create a new middleware to serve as a router.
    ///
    ///
    /// # Examples
    /// ```{rust}
    /// extern crate nickel;
    /// #[macro_use] extern crate nickel_macros;
    /// use nickel::{Nickel, HttpRouter};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     let mut router = Nickel::router();
    ///
    ///     router.get("/foo", middleware! {
    ///         "Hi from /foo"
    ///     });
    ///
    ///     server.utilize(router);
    /// }
    /// ```
    pub fn router() -> Router {
        Router::new()
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Examples
    /// ```{rust,no_run}
    /// use nickel::Nickel;
    /// use std::net::IpAddr;
    ///
    /// let mut server = Nickel::new();
    /// server.listen(IpAddr::new_v4(127, 0, 0, 1), 6767);
    /// ```
    pub fn listen(mut self, ip: IpAddr, port: u16) {
        self.middleware_stack.add_middleware(middleware! {
            (StatusCode::NotFound, "File Not Found")
        });

        match port {
            80u16 =>  println!("Listening on http://{}", ip),
            _ =>  println!("Listening on http://{}:{}", ip, port),
        }
        println!("Ctrl-C to shutdown server");

        Server::new(self.middleware_stack).serve(ip, port);
    }
}
