use std::old_io::net::ip::{Port, IpAddr};

use request::Request;
use response::Response;
use router::{Router, HttpRouter};
use middleware::{MiddlewareResult, MiddlewareStack, Middleware, ErrorHandler};
use nickel_error::{ErrorWithStatusCode, NickelError};
use server::Server;

use hyper::method::Method;
use hyper::status::StatusCode::NotFound;
use request::Request;
use response::Response;

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
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, Continue, MiddlewareResult};
    /// fn logger(req: &Request, res: &mut Response) -> MiddlewareResult {
    ///     println!("logging request: {}", req.origin.uri);
    ///     Ok(Continue)
    /// }
    ///
    /// let mut server = Nickel::new();
    /// let l: fn(&Request, &mut Response) -> MiddlewareResult = logger;
    /// server.utilize(l);
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
    /// ```{rust}
    /// # extern crate http;
    /// # extern crate nickel;
    /// # fn main() {
    /// use nickel::{Nickel, Request, Response, Continue, Halt, MiddlewareResult};
    /// use nickel::{NickelError, ErrorWithStatusCode};
    /// use hyper::status::StatusCode::NotFound;
    /// use nickel::mimes::MediaType::Html;
    ///
    /// fn error_handler(err: &NickelError, req: &Request, response: &mut Response)
    ///                  -> MiddlewareResult {
    ///    match err.kind {
    ///        ErrorWithStatusCode(NotFound) => {
    ///            response.content_type(Html);
    ///            response.origin.status = NotFound;
    ///            response.send("<h1>Call the police!<h1>");
    ///            Ok(Halt)
    ///        },
    ///        _ => Ok(Continue)
    ///    }
    /// }
    ///
    /// let mut server = Nickel::new();
    ///
    /// let ehandler: fn(&NickelError, &Request, &mut Response) -> MiddlewareResult = error_handler;
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
    /// let fhandler: fn(&Request, &mut Response) = foo_handler;
    ///
    /// router.get("/foo", fhandler);
    ///
    /// server.utilize(router);
    /// ```
    pub fn router() -> Router {
        Router::new()
    }

    /// Bind and listen for connections on the given host and port
    ///
    /// # Example
    /// ```{rust,no_run}
    /// use nickel::Nickel;
    /// use std::old_io::net::ip::IpAddr::Ipv4Addr;
    ///
    /// let mut server = Nickel::new();
    /// server.listen(Ipv4Addr(127, 0, 0, 1), 6767);
    /// ```
    pub fn listen(mut self, ip: IpAddr, port: Port) {
        fn not_found_handler(_: &Request, _: &mut Response) -> MiddlewareResult {
            Err(NickelError::new("File Not Found", ErrorWithStatusCode(NotFound)))
        }

        let nfhandler: fn(&Request, &mut Response) -> MiddlewareResult = not_found_handler;
        self.middleware_stack.add_middleware(nfhandler);

        match port {
            80u16 =>  println!("Listening on http://{}", ip),
            _ =>  println!("Listening on http://{}:{}", ip, port),
        }
        println!("Ctrl-C to shutdown server");

        Server::new(self.middleware_stack, ip, port).serve();
    }
}
