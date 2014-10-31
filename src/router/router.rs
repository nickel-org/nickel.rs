use middleware::{Middleware, Continue, MiddlewareResult};
use super::path_utils;
use http::server::request::AbsolutePath;
use request::Request;
use response::Response;
use router::{HttpRouter, RequestHandler};
use http::method::Method;
use regex::Regex;
use std::collections::hashmap::HashMap;

/// A Route is the basic data structure that stores both the path
/// and the handler that gets executed for the route.
/// The path can contain variable pattern such as `user/:userid/invoices`
pub trait RouteTrait: Send + Sync {
    fn get_path(&self) -> &String;
    fn get_method(&self) -> &Method;
    fn handle(&self, &Request, &mut Response) -> MiddlewareResult;
    fn get_variables(&self) -> &HashMap<String, uint>;
    fn get_matcher(&self) -> &Regex;
}

pub struct Route<T: Send + Sync> {
    pub path: String,
    pub method: Method,
    pub handler: Box<RequestHandler<T> + Send + Sync + 'static>,
    pub variables: HashMap<String, uint>,
    pub route_data: T,
    matcher: Regex
}

impl<T: Send + Sync> RouteTrait for Route<T> {
    fn get_path(&self) -> &String {
        &self.path
    }
    
    fn get_method(&self) -> &Method {
        &self.method
    }

    fn handle(&self, req: &Request, res: &mut Response) -> MiddlewareResult {
        self.handler.handle(req, res, &self.route_data)
    }

    fn get_variables(&self) -> &HashMap<String, uint> {
        &self.variables
    }
    
    fn get_matcher(&self) -> &Regex {
        &self.matcher
    }
}


/// A RouteResult is what the router returns when `match_route` is called.
/// It contains the matched `route` and also a `params` property holding
/// a HashMap with the keys being the variable names and the value being the
/// evaluated string
pub struct RouteResult<'a> {
    pub route: &'a RouteTrait,
    params: Vec<String>
}

impl<'a> RouteResult<'a> {
    pub fn param(&self, key: &str) -> &str {
        let idx = self.route.get_variables().find_equiv(&key).unwrap();
        self.params[*idx].as_slice()
    }
}

/// The Router's job is it to hold routes and to resolve them later against
/// concrete URLs. The router is also a regular middleware and needs to be
/// added to the middleware stack with `server.utilize(router)`.
pub struct Router{
    //for some reason we have to give the compiler some help
    //with the traits here, even though RouteTrait should imply
    //Sned + Sync. Looks like https://github.com/rust-lang/rust/issues/15155
    routes: Vec<Box<RouteTrait + Send + Sync + 'static>>,
}

impl<'a> Router {
    pub fn new () -> Router {
        Router {
            routes: Vec::new()
        }
    }

    pub fn match_route(&'a self, method: &Method, path: &str) -> Option<RouteResult<'a>> {
        self.routes.iter().find(|item| *item.get_method() == *method && item.get_matcher().is_match(path))
            .map(|route| {
                let vec = match route.get_matcher().captures(path) {
                    Some(captures) => {
                        range(0, route.get_variables().len()).map(|pos|
                            captures.at(pos + 1).to_string()
                        ).collect()
                    },
                    None => vec![],
                };
                RouteResult {
                    route: &**route,
                    params: vec
                }
            })
    }
}

impl HttpRouter for Router {
    fn add_route<H: RequestHandler<()>>(&mut self, method: Method, path: &str, handler: H) {
        let matcher = path_utils::create_regex(path);
        let variable_infos = path_utils::get_variable_info(path);
        let route = box Route {
            path: path.to_string(),
            method: method,
            matcher: matcher,
            handler: box handler,
            route_data: {},
            variables: variable_infos
        };
        self.routes.push(route);
    }
  
    fn add_route_with_data<T: Send + Sync, H: RequestHandler<T>>(&mut self, method: Method, path: &str, handler: H, route_data: T) {
        let matcher = path_utils::create_regex(path);
        let variable_infos = path_utils::get_variable_info(path);
        let route = box Route {
            path: path.to_string(),
            method: method,
            matcher: matcher,
            handler: box handler,
            route_data: route_data,
            variables: variable_infos
        };
        self.routes.push(route);
    }
}

impl Middleware for Router {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, res: &mut Response)
                        -> MiddlewareResult {
        match req.origin.request_uri {
            AbsolutePath(ref url) => {
                match self.match_route(&req.origin.method, url.as_slice()) {
                    Some(route_result) => {
                        res.origin.status = ::http::status::Ok;
                        let route = route_result.route;
                        req.route_result = Some(route_result);
                        route.handle(req, res)
                    },
                    None => Ok(Continue)
                }
            },
            _ => Ok(Continue)
        }
    }
}

#[test]
fn creates_map_with_var_variable_infos () {
    let map = path_utils::get_variable_info("foo/:uid/bar/:groupid");

    assert_eq!(map.len(), 2);
    assert_eq!(map["uid".to_string()], 0);
    assert_eq!(map["groupid".to_string()], 1);
}

#[test]
fn creates_regex_with_captures () {
    let regex = path_utils::create_regex("foo/:uid/bar/:groupid");
    let caps = regex.captures("foo/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");

    let regex = path_utils::create_regex("foo/*/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");

    let regex = path_utils::create_regex("foo/**/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/another/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");
}

#[test]
fn creates_valid_regex_for_routes () {
    let regex1 = path_utils::create_regex("foo/:uid/bar/:groupid");
    let regex2 = path_utils::create_regex("foo/*/bar");
    let regex3 = path_utils::create_regex("foo/**/bar");

    assert_eq!(regex1.is_match("foo/4711/bar/5490"), true);
    assert_eq!(regex1.is_match("foo/4711/bar/5490?foo=true&bar=false"), true);
    assert_eq!(regex1.is_match("foo/4711/bar"), false);
    assert_eq!(regex1.is_match("foo/4711/bar?foo=true&bar=false"), false);
    assert_eq!(regex1.is_match("foo/4711/bar/test%20spacing"), true);
    assert_eq!(regex1.is_match("foo/4711/bar/5281?foo=test%20spacing&bar=false"), true);

    assert_eq!(regex2.is_match("foo/4711/bar"), true);
    assert_eq!(regex2.is_match("foo/4711/barr"), false);
    assert_eq!(regex2.is_match("foo/4711/bar?foo=true&bar=false"), true);
    assert_eq!(regex2.is_match("foo/4711/4712/bar"), false);
    assert_eq!(regex2.is_match("foo/4711/4712/bar?foo=true&bar=false"), false);

    assert_eq!(regex3.is_match("foo/4711/bar"), true);
    assert_eq!(regex3.is_match("foo/4711/bar?foo=true&bar=false"), true);
    assert_eq!(regex3.is_match("foo/4711/4712/bar"), true);
    assert_eq!(regex3.is_match("foo/4711/4712/bar?foo=true&bar=false"), true);

    //ensure that this works with commas too
    assert_eq!(regex1.is_match("foo/4711/bar/5490,1234"), true);
    assert_eq!(regex1.is_match("foo/4711/bar/5490,1234?foo=true&bar=false"), true);
    assert_eq!(regex1.is_match("foo/4711/bar"), false);
    assert_eq!(regex1.is_match("foo/4711/bar?foo=1,2,3&bar=false"), false);
}

#[test]
fn can_match_var_routes () {
    use http::method;
    use request::Request;
    use response::Response;

    let route_store = &mut Router::new();

    fn handler (_request: &Request, response: &mut Response) -> () {
        let _ = response.origin.write("hello from foo".as_bytes());
    };

    route_store.add_route(method::Get, "/foo/:userid", handler);
    route_store.add_route(method::Get, "/bar", handler);

    let route_result = route_store.match_route(&method::Get, "/foo/4711").unwrap();
    let route = route_result.route;

    assert_eq!(route_result.param("userid"), "4711");

    //assert the route has identified the variable
    assert_eq!(route.get_variables().len(), 1);
    assert_eq!(route.get_variables()["userid".to_string()], 0);

    let route_result = route_store.match_route(&method::Get, "/bar/4711");
    assert!(route_result.is_none());

    let route_result = route_store.match_route(&method::Get, "/foo");
    assert!(route_result.is_none());

    //ensure that this will work with commas too
    let route_result = route_store.match_route(&method::Get, "/foo/123,456");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), "123,456");

    //ensure that this will work with spacing too
    let route_result = route_store.match_route(&method::Get, "/foo/John%20Doe");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), "John%20Doe");
}

