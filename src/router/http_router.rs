use hyper::method::Method;
use middleware::Middleware;
use router::Matcher;

pub trait HttpRouter<D> {
    /// Registers a handler to be used for a specified method.
    /// A handler can be anything implementing the `RequestHandler` trait.
    ///
    /// # Examples
    ///
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// extern crate hyper;
    /// extern crate regex;
    ///
    /// use nickel::{Nickel, HttpRouter};
    /// use hyper::method::Method::{Get, Post, Put, Delete};
    /// use regex::Regex;
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///
    ///     server.add_route(Get, "/foo", middleware! { "Get request! "});
    ///     server.add_route(Post, "/foo", middleware! { |request|
    ///         format!("Method is: {}", request.origin.method)
    ///     });
    ///     server.add_route(Put, "/foo", middleware! { |request|
    ///         format!("Method is: {}", request.origin.method)
    ///     });
    ///     server.add_route(Delete, "/foo", middleware! { |request|
    ///         format!("Method is: {}", request.origin.method)
    ///     });
    ///
    ///     // Regex path
    ///     let regex = Regex::new("/(foo|bar)").unwrap();
    ///     server.add_route(Get, regex, middleware! { "Regex Get request! "});
    /// }
    /// ```
    fn add_route<M: Into<Matcher>, H: Middleware<D>>(&mut self, Method, M, H);

    /// Registers a handler to be used for a specific GET request.
    /// Handlers are assigned to paths and paths are allowed to contain
    /// variables and wildcards.
    ///
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// # Examples
    ///
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// use nickel::{Nickel, HttpRouter};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///
    ///     //  without variables or wildcards
    ///     server.get("/user", middleware! { "This matches /user" });
    ///
    ///     // with variables
    ///     server.get("/user/:userid", middleware! { |request|
    ///         format!("This is user: {}", request.param("userid").unwrap())
    ///     });
    ///
    ///     // with simple wildcard
    ///     server.get("/user/*/:userid", middleware! {
    ///         "This matches /user/list/4711 but not /user/extended/list/4711"
    ///     });
    ///
    ///     // with double wildcard
    ///     server.get("/user/**/:userid", middleware! {
    ///         "This matches /user/list/4711 and also /user/extended/list/4711"
    ///     });
    /// }
    /// ```
    ///
    /// # router! macro example
    ///
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// use nickel::Nickel;
    ///
    /// fn main() {
    ///     let router = router! {
    ///         //  without variables or wildcards
    ///         get "/user" => |_, response| {
    ///             "This matches /user";
    ///         }
    ///         // with variables
    ///         get "/user/:userid" => |request, response| {
    ///             format!("This is user: {}", request.param("userid").unwrap())
    ///         }
    ///         // with simple wildcard
    ///         get "/user/*/:userid" => |_, response| {
    ///             ["This matches /user/list/4711",
    ///              "NOT /user/extended/list/4711"];
    ///         }
    ///         // with double wildcard
    ///         get "/user/**/:userid" => |_, response| {
    ///             ["This matches /user/list/4711",
    ///              "AND /user/extended/list/4711"];
    ///         }
    ///     };
    ///
    ///     let mut server = Nickel::new();
    ///     server.utilize(router);
    /// }
    /// ```
    fn get<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Get, matcher, handler);
    }

    /// Registers a handler to be used for a specific POST request.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    fn post<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Post, matcher, handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    fn put<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Put, matcher, handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    fn delete<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Delete, matcher, handler);
    }

    /// Registers a handler to be used for a specific OPTIONS request.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    fn options<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Options, matcher, handler);
    }

    /// Registers a handler to be used for a specific PATCH request.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    fn patch<M: Into<Matcher>, H: Middleware<D>>(&mut self, matcher: M, handler: H) {
        self.add_route(Method::Patch, matcher, handler);
    }
}
