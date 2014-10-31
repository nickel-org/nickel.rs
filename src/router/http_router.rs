use http::method::{ Method, Get, Post, Put, Delete };
use router::RequestHandler;

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
    ///     server.add_route(Get, "/foo", read_handler);
    ///     server.add_route(Post, "/foo", modify_handler);
    ///     server.add_route(Put, "/foo", modify_handler);
    ///     server.add_route(Delete, "/foo", modify_handler);
    /// }
    /// ```
    fn add_route<H: RequestHandler<()>>(&mut self, Method, &str, H);
    fn add_route_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, Method, &str, H, T);

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
    ///
    /// # Macro example
    ///
    /// ```{rust}
    /// # #![feature(phase)]
    /// #[phase(plugin)] extern crate nickel_macros;
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
    fn get<H: RequestHandler<()>>(&mut self, uri: &str, handler: H) {
        self.add_route(Get, uri, handler);
    }
    fn get_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, uri: &str, handler: H, route_data: T) {
        self.add_route_with_data(Get, uri, handler, route_data);
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
    /// server.post("/a/post/request", handler);
    /// ```
    fn post<H: RequestHandler<()>>(&mut self, uri: &str, handler: H) {
        self.add_route(Post, uri, handler);
    }
    fn post_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, uri: &str, handler: H, route_data: T) {
        self.add_route_with_data(Post, uri, handler, route_data);
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
    /// server.put("/a/put/request", handler);
    /// ```
    fn put<H: RequestHandler<()>>(&mut self, uri: &str, handler: H) {
        self.add_route(Put, uri, handler);
    }
    fn put_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, uri: &str, handler: H, route_data: T) {
        self.add_route_with_data(Put, uri, handler, route_data);
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
    /// server.delete("/a/delete/request", handler);
    /// ```
    fn delete<H: RequestHandler<()>>(&mut self, uri: &str, handler: H) {
        self.add_route(Delete, uri, handler);
    }
    fn delete_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, uri: &str, handler: H, route_data: T) {
        self.add_route_with_data(Delete, uri, handler, route_data);
    }
}
