use hyper::method::Method;
use middleware::Middleware;

pub trait HttpRouter {
    /// Registers a handler to be used for a specified method.
    /// A handler can be anything implementing the `RequestHandler` trait.
    ///
    /// # Examples
    ///
    /// ```{rust}
    /// extern crate hyper;
    /// extern crate nickel;
    /// #[macro_use] extern crate nickel_macros;
    ///
    /// use nickel::{Nickel, HttpRouter};
    /// use hyper::method::Method::{Get, Post, Put, Delete};
    ///
    /// fn main() {
    ///     let read_handler = middleware! { "Get request! "};
    ///     let modify_handler = middleware! { |request|
    ///         format!("Method is: {}", request.origin.method)
    ///     };
    ///
    ///     let mut server = Nickel::new();
    ///
    ///     server.add_route(Get, "/foo", read_handler);
    ///     server.add_route(Post, "/foo", modify_handler);
    ///     server.add_route(Put, "/foo", modify_handler);
    ///     server.add_route(Delete, "/foo", modify_handler);
    /// }
    /// ```
    fn add_route<H: Middleware>(&mut self, Method, &str, H);

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
    /// extern crate nickel;
    /// #[macro_use] extern crate nickel_macros;
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///
    ///     //  without variables or wildcards
    ///     server.get("/user", middleware! { "This matches /user" });
    ///
    ///     // with variables
    ///     server.get("/user/:userid", middleware! { |request|
    ///         format!("This is user: {}", request.param("userid"))
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
    /// #![feature(plugin)]
    /// #[macro_use] extern crate nickel_macros;
    /// extern crate nickel;
    /// use nickel::Nickel;
    ///
    /// fn main() {
    ///     let router = router! {
    ///         //  without variables or wildcards
    ///         get "/user" => |request, response| {
    ///             "This matches /user";
    ///         }
    ///         // with variables
    ///         get "/user/:userid" => |request, response| {
    ///             format!("This is user: {}", request.param("userid"))
    ///         }
    ///         // with simple wildcard
    ///         get "/user/*/:userid" => |request, response| {
    ///             ["This matches /user/list/4711",
    ///              "NOT /user/extended/list/4711"];
    ///         }
    ///         // with double wildcard
    ///         get "/user/**/:userid" => |request, response| {
    ///             ["This matches /user/list/4711",
    ///              "AND /user/extended/list/4711"];
    ///         }
    ///     };
    ///
    ///     let mut server = Nickel::new();
    ///     server.utilize(router);
    /// }
    /// ```
    fn get<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Method::Get, uri, handler);
    }

    /// Registers a handler to be used for a specific POST request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Examples
    ///
    /// ```{rust}
    /// # extern crate nickel;
    /// # #[macro_use] extern crate nickel_macros;
    /// # fn main() {
    /// use nickel::{Nickel, HttpRouter};
    ///
    /// let mut server = Nickel::new();
    /// server.post("/a/post/request", middleware! {
    ///     "This matches a POST request to /a/post/request"
    /// });
    /// # }
    /// ```
    fn post<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Method::Post, uri, handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Examples
    ///
    /// ```{rust}
    /// # extern crate nickel;
    /// # #[macro_use] extern crate nickel_macros;
    /// # fn main() {
    /// use nickel::{Nickel, HttpRouter};
    ///
    /// let mut server = Nickel::new();
    /// server.put("/a/put/request", middleware! {
    ///     "This matches a PUT request to /a/put/request"
    /// });
    /// # }
    /// ```
    fn put<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Method::Put, uri, handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Examples
    /// ```{rust}
    /// # extern crate nickel;
    /// # #[macro_use] extern crate nickel_macros;
    /// # fn main() {
    /// use nickel::{Nickel, HttpRouter};
    ///
    /// let mut server = Nickel::new();
    /// server.delete("/a/delete/request", middleware! {
    ///     "This matches a DELETE request to /a/delete/request"
    /// });
    /// # }
    /// ```
    fn delete<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Method::Delete, uri, handler);
    }
}
