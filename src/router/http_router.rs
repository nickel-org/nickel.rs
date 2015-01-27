use http::method::{Method, Get, Post, Put, Delete};
use middleware::Middleware;

pub trait HttpRouter {
    /// Registers a handler to be used for a specified method.
    /// A handler can be anything implementing the `RequestHandler` trait.
    ///
    /// # Example
    ///
    /// ```{rust}
    /// extern crate nickel;
    /// extern crate http;
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    /// use http::method::{Get, Post, Put, Delete};
    ///
    /// fn main() {
    ///     fn read_handler(request: &Request, response: &mut Response) {
    ///         response.send("Get request!");
    ///     };
    ///     fn modify_handler(request: &Request, response: &mut Response) {
    ///         response.send(format!("Method is: {}", request.origin.method));
    ///     };
    ///
    ///     let mut server = Nickel::new();
    ///
    ///     let rhandler: fn(&Request, &mut Response) = read_handler;
    ///     server.add_route(Get, "/foo", rhandler);
    ///
    ///     let mhandler: fn(&Request, &mut Response) = modify_handler;
    ///     server.add_route(Post, "/foo", mhandler);
    ///     server.add_route(Put, "/foo", mhandler);
    ///     server.add_route(Delete, "/foo", mhandler);
    /// }
    /// ```
    fn add_route<H: Middleware>(&mut self, Method, &str, H);

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
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    /// let mut server = Nickel::new();
    ///
    /// //  without variables or wildcards
    /// fn bare_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user");
    /// };
    /// let bhandler: fn(&Request, &mut Response) = bare_handler;
    /// server.get("/user", bhandler);
    ///
    /// // with variables
    /// fn var_handler(request: &Request, response: &mut Response) {
    ///     let text = format!("This is user: {}", request.param("userid"));
    ///     response.send(text.as_slice());
    /// };
    /// let vhandler: fn(&Request, &mut Response) = var_handler;
    /// server.get("/user/:userid", vhandler);
    ///
    /// // with simple wildcard
    /// fn wild_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 but not /user/extended/list/4711");
    /// };
    /// let whandler: fn(&Request, &mut Response) = wild_handler;
    /// server.get("/user/*/:userid", whandler);
    ///
    /// // with double wildcard
    /// fn very_wild_handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches /user/list/4711 and also /user/extended/list/4711");
    /// };
    /// let vwhandler: fn(&Request, &mut Response) = very_wild_handler;
    /// server.get("/user/**/:userid", vwhandler);
    /// ```
    ///
    /// # Macro example
    ///
    /// ```{rust}
    /// #![feature(plugin)]
    /// #[plugin] #[macro_use] extern crate nickel_macros;
    /// extern crate nickel;
    /// use nickel::Nickel;
    ///
    /// fn main() {
    ///     let router = router! {
    ///         //  without variables or wildcards
    ///         get "/user" => |request, response| {
    ///             response.send("This matches /user");
    ///         }
    ///         // with variables
    ///         get "/user/:userid" => |request, response| {
    ///             let text = format!("This is user: {}", request.param("userid"));
    ///             response.send(text.as_slice());
    ///         }
    ///         // with simple wildcard
    ///         get "/user/*/:userid" => |request, response| {
    ///             response.send("This matches /user/list/4711");
    ///             response.send("NOT /user/extended/list/4711");
    ///         }
    ///         // with double wildcard
    ///         get "/user/**/:userid" => |request, response| {
    ///             response.send("This matches /user/list/4711");
    ///             response.send("AND /user/extended/list/4711");
    ///         }
    ///     };
    ///
    ///     let mut server = Nickel::new();
    ///     server.utilize(router);
    /// }
    /// ```
    fn get<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Get, uri, handler);
    }

    /// Registers a handler to be used for a specific POST request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/post/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// let h: fn(&Request, &mut Response) = handler;
    /// server.post("/a/post/request", h);
    /// ```
    fn post<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Post, uri, handler);
    }

    /// Registers a handler to be used for a specific PUT request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a POST request to /a/put/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// let h: fn(&Request, &mut Response) = handler;
    /// server.put("/a/put/request", h);
    /// ```
    fn put<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Put, uri, handler);
    }

    /// Registers a handler to be used for a specific DELETE request.
    /// A handler added through this API will be attached to the default router.
    /// Consider creating the router middleware manually for advanced functionality.
    ///
    /// Take a look at `get(...)` for a more detailed description.
    /// # Example
    ///
    /// ```{rust}
    /// use nickel::{Nickel, Request, Response, HttpRouter};
    /// fn handler(request: &Request, response: &mut Response) {
    ///     response.send("This matches a DELETE request to /a/delete/request");
    /// };
    ///
    /// let mut server = Nickel::new();
    /// let h: fn(&Request, &mut Response) = handler;
    /// server.delete("/a/delete/request", h);
    /// ```
    fn delete<H: Middleware>(&mut self, uri: &str, handler: H) {
        self.add_route(Delete, uri, handler);
    }
}
