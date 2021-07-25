use std::net::ToSocketAddrs;
use std::time::Duration;
use std::env;
use std::error::Error as StdError;
use crate::router::{Router, HttpRouter, Matcher};
use crate::middleware::{MiddlewareStack, Middleware, ErrorHandler};
use crate::server::Server;
use crate::template_cache::ReloadPolicy;
use hyper::{Method, StatusCode};
//use hyper::net::SslServer;

//pre defined middleware
use crate::default_error_handler::DefaultErrorHandler;

/// Configuration options for the server.
///
/// This struct provides a builder-style API for constructing the desired options.
///
/// # Examples
/// ```{rust}
/// use nickel::{Nickel, Options};
/// let mut server = Nickel::new();
///
/// // Don't print to stdout when starting the server
/// // and force using 8 threads.
/// server.options = Options::default()
///                      .output_on_listen(false)
///                      .thread_count(Some(8));
/// ```
pub struct Options {
    output_on_listen: bool,
    thread_count: Option<usize>,
    reload_policy: ReloadPolicy,
}

impl Options {
    /// Whether the server should print the local address it is listening on when starting.
    ///
    /// Defaults to `true`.
    pub fn output_on_listen(mut self, output: bool) -> Self {
        self.output_on_listen = output;
        self
    }

    /// The number of threads the server should use or `None` to use the
    /// `hyper`'s default of `1.25 * core_count`.
    ///
    /// Defaults to `None`.
    pub fn thread_count(mut self, thread_count: Option<usize>) -> Self {
        self.thread_count = thread_count;
        self
    }

    /// The TemplateCache reload policy. Defaults to ReloadPolicy::Never.
    pub fn reload_policy(mut self, reload_policy: ReloadPolicy) -> Self {
        self.reload_policy = reload_policy;
        self
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            output_on_listen: true,
            thread_count: None,
            reload_policy: ReloadPolicy::Never,
        }
    }
}

/// Nickel is the application object. It's the surface that
/// holds all public APIs.
pub struct Nickel<D: Sync + Send + 'static = ()> {
    middleware_stack: MiddlewareStack<D>,
    data: D,
    keep_alive_timeout: Option<Duration>,

    /// Configuration options for the server.
    pub options: Options,
}

impl<D: Sync + Send + 'static> HttpRouter<D> for Nickel<D> {
    fn add_route<M: Into<Matcher>, H: Middleware<D>>(&mut self, method: Method, matcher: M, handler: H) -> &mut Self {
        let mut router = Router::new();
        router.add_route(method, matcher, handler);
        self.utilize(router);
        self
    }
}

impl Nickel<()> {
    /// Creates an instance of Nickel with default error handling.
    pub fn new() -> Nickel<()> {
        Nickel::with_data(())
    }

    /// Creates and instance of Nickel with custom Options.
    pub fn with_options(options: Options) -> Nickel<()> {
        Nickel::with_data_and_options((), options)
    }
}

impl<D: Sync + Send + 'static> Nickel<D> {
    /// Creates an instance of Nickel with default error handling,
    /// custom options, and custom data.
    pub fn with_data_and_options(data: D, options: Options) -> Nickel<D> {
        let mut middleware_stack = MiddlewareStack::new();

        // Hook up the default error handler by default. Users are
        // free to cancel it out from their custom error handler if
        // they don't like the default behaviour.
        middleware_stack.add_error_handler(DefaultErrorHandler);

        Nickel {
            middleware_stack: middleware_stack,
            options: options,
            data: data,
            // Default value from nginx
            keep_alive_timeout: Some(Duration::from_secs(75)),
        }
    }

    /// Creates an instance of Nickel with default error handling,
    /// default options, and custom data.
    pub fn with_data(data: D) -> Nickel<D> {
        Nickel::with_data_and_options(data, Options::default())
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
    /// # #[macro_use] extern crate nickel;
    /// # fn main() {
    /// use nickel::Nickel;
    /// let mut server = Nickel::new();
    /// server.utilize(middleware! { |req|
    ///     println!("logging request: {:?}", req.origin.uri());
    /// });
    /// # }
    /// ```
    pub fn utilize<T: Middleware<D>>(&mut self, handler: T){
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
    /// use nickel::{Nickel, Request, Continue, Halt};
    /// use nickel::{NickelError, Action};
    /// use nickel::status::StatusCode;
    ///
    /// fn error_handler<D: Send + Sync + 'static>(err: &mut NickelError<D>, _req: &mut Request<D>) -> Action {
    ///    if let Some(ref mut res) = err.stream {
    ///        if res.status() == StatusCode::NOT_FOUND {
    ///            let _ = res.set_body("<h1>Call the police!</h1>");
    ///            return Halt(())
    ///        }
    ///    }
    ///
    ///     Continue(())
    /// }
    ///
    /// let mut server = Nickel::new();
    ///
    /// let ehandler: fn(&mut NickelError<()>, &mut Request<()>) -> Action = error_handler;
    ///
    /// server.handle_error(ehandler)
    /// # }
    /// ```
    pub fn handle_error<T: ErrorHandler<D>>(&mut self, handler: T){
        self.middleware_stack.add_error_handler(handler);
    }

    /// Create a new middleware to serve as a router.
    ///
    ///
    /// # Examples
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
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
    pub fn router() -> Router<D> {
        Router::new()
    }

    /// Bind and listen for connections on the given host and port.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use nickel::Nickel;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let server = Nickel::new();
    ///     server.listen("127.0.0.1:6767").await.expect("Failed to launch server");
    /// }
    /// ```
    pub async fn listen<T: ToSocketAddrs>(mut self, addr: T) -> Result<(), Box<dyn StdError>> {
        self.middleware_stack.add_middleware(middleware! {
            (StatusCode::NOT_FOUND, "File Not Found")
        });

        let server = Server::new(self.middleware_stack, self.options.reload_policy, self.data);

        let is_test_harness = env::var_os("NICKEL_TEST_HARNESS").is_some();

        if is_test_harness {
            // If we're under a test harness, we'll pass zero to get assigned a random
            // port. See http://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
            server.serve("localhost:0",
                         self.keep_alive_timeout,
                         self.options.thread_count).await?
        } else {
            // TODO: fixme
            // if self.options.output_on_listen {
            //     println!("Listening on http://{}", addr);
            // }
            server.serve(addr,
                         self.keep_alive_timeout,
                         self.options.thread_count).await?
        };

        if self.options.output_on_listen {
            println!("Ctrl-C to shutdown server");
        }

        Ok(())
    }

    /// Set the timeout for the keep-alive loop
    ///
    /// # Performance
    ///
    /// Setting this to `None` can have significant performance impact, but if
    /// you need to use a version of rustc < 1.4, then it may be a good choice.
    ///
    /// Alternatively, setting this too high, can lead to thread exhaustion,
    /// see [this thread](https://github.com/hyperium/hyper/issues/368) for more.
    ///
    /// # Default
    ///
    /// The default value is 75 seconds.
    pub fn keep_alive_timeout(&mut self, timeout: Option<Duration>){
        self.keep_alive_timeout = timeout;
    }

    /*
    /// Bind and listen for connections on the given host and port.
    /// Only accepts SSL connections
    ///
    /// # Panics
    ///
    /// Panics if `addr` is an invalid `SocketAddr`.
    ///
    /// # Examples
    /// ```{rust,no_run}
    /// # extern crate nickel;
    /// extern crate hyper;
    ///
    /// # #[cfg(feature = "ssl")]
    /// use nickel::Nickel;
    /// # #[cfg(feature = "ssl")]
    /// use hyper::net::Openssl;
    ///
    /// # #[cfg(feature = "ssl")]
    /// # fn main() {
    /// let server = Nickel::new();
    /// let ssl = Openssl::with_cert_and_key("foo.crt", "key.pem").unwrap();
    /// server.listen_https("127.0.0.1:6767", ssl).unwrap();
    /// # }
    /// # #[cfg(not(feature = "ssl"))]
    /// # fn main() {}
    /// ```
     */
    //
    // Ssl support changed in hyper 0.11
    //
    // pub fn listen_https<T,S>(mut self, addr: T, ssl: S) -> Result<ListeningServer, Box<dyn StdError>>
    // where T: ToSocketAddrs,
    //       S: SslServer + Send + Clone + 'static {
    //     self.middleware_stack.add_middleware(middleware! {
    //         (StatusCode::NOT_FOUND, "File Not Found")
    //     });

    //     let server = Server::new(self.middleware_stack, self.options.reload_policy, self.data);

    //     let is_test_harness = env::var_os("NICKEL_TEST_HARNESS").is_some();

    //     let listener = if is_test_harness {
    //         // If we're under a test harness, we'll pass zero to get assigned a random
    //         // port. See http://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
    //         server.serve_https("localhost:0",
    //                            self.keep_alive_timeout,
    //                            self.options.thread_count,
    //                            ssl)?
    //     } else {
    //         server.serve_https(addr,
    //                            self.keep_alive_timeout,
    //                            self.options.thread_count,
    //                            ssl)?
    //     };

    //     if self.options.output_on_listen {
    //         println!("Listening on https://{}", listener.socket());
    //         println!("Ctrl-C to shutdown server");
    //     }

    //     Ok(listener)
    // }
}

#[cfg(test)]
mod tests {
    use crate::Nickel;
    use std::str::FromStr;
    use std::net::SocketAddr;

    #[tokio::test]
    #[should_panic(expected = "invalid socket address")]
    async fn invalid_listen_addr() {
        Nickel::new().listen("127.0.0.1.6667").await.unwrap();
    }

    // TODO: These tests are commented out since we no longher provide access to
    // the underlying server. Do we need to?

    // #[test]
    // fn can_get_server_address() {
    //     let server = Nickel::new().listen("127.0.0.1:12345").await.unwrap();

    //     assert_eq!(server.socket(), SocketAddr::from_str("127.0.0.1:12345").unwrap());
    //     server.detach();
    // }

    // #[test]
    // fn can_get_server_address_with_random_port() {
    //     let server = Nickel::new().listen("127.0.0.1:0").await.unwrap();

    //     assert_eq!(server.socket().ip().to_string(), "127.0.0.1");
    //     server.detach();
    // }
}
