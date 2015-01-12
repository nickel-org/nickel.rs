use middleware::{Middleware, Continue, MiddlewareResult};
use super::path_utils;
use hyper::uri::RequestUri::AbsolutePath;
use request::Request;
use response::Response;
use router::HttpRouter;
use hyper::method::Method;
use hyper::status::StatusCode;
use regex::Regex;
use std::collections::HashMap;

/// A Route is the basic data structure that stores both the path
/// and the handler that gets executed for the route.
/// The path can contain variable pattern such as `user/:userid/invoices`
pub struct Route {
    pub path: String,
    pub method: Method,
    pub handler: Box<Middleware + Send + Sync + 'static>,
    pub variables: HashMap<String, usize>,
    matcher: Regex
}

/// A RouteResult is what the router returns when `match_route` is called.
/// It contains the matched `route` and also a `params` property holding
/// a HashMap with the keys being the variable names and the value being the
/// evaluated string
pub struct RouteResult<'a> {
    pub route: &'a Route,
    params: Vec<String>
}

impl<'a> RouteResult<'a> {
    pub fn param(&self, key: &str) -> &str {
        let idx = self.route.variables.get(key).unwrap();
        &*self.params[*idx]
    }
}

/// The Router's job is it to hold routes and to resolve them later against
/// concrete URLs. The router is also a regular middleware and needs to be
/// added to the middleware stack with `server.utilize(router)`.
pub struct Router {
    routes: Vec<Route>,
}

impl<'a> Router {
    pub fn new () -> Router {
        Router {
            routes: Vec::new()
        }
    }

    pub fn match_route(&'a self, method: &Method, path: &str) -> Option<RouteResult<'a>> {
        self.routes
            .iter()
            .find(|item| item.method == *method && item.matcher.is_match(path))
            .map(|route| {
                let vec = match route.matcher.captures(path) {
                    Some(captures) => {
                        range(0, route.variables.len())
                            .filter_map(|pos| {
                                captures.at(pos + 1).map(|c| c.to_string())
                            })
                            .collect()
                    },
                    None => vec![],
                };
                RouteResult {
                    route: route,
                    params: vec
                }
            })
    }
}

impl HttpRouter for Router {
    fn add_route<H: Middleware>(&mut self, method: Method, path: &str, handler: H) {
        let matcher = path_utils::create_regex(path);
        let variable_infos = path_utils::get_variable_info(path);
        let route = Route {
            path: path.to_string(),
            method: method,
            matcher: matcher,
            handler: Box::new(handler),
            variables: variable_infos
        };
        self.routes.push(route);
    }
}

impl Middleware for Router {
    fn invoke<'a, 'b>(&'a self, req: &mut Request<'b, 'a>, res: &mut Response)
                        -> MiddlewareResult {
        match req.origin.uri {
            AbsolutePath(ref url) => {
                match self.match_route(&req.origin.method, &*url) {
                    Some(route_result) => {
                        res.origin.status = StatusCode::Ok;
                        let handler = &route_result.route.handler;
                        req.route_result = Some(route_result);
                        handler.invoke(req, res)
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

    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");

    let regex = path_utils::create_regex("foo/*/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");

    let regex = path_utils::create_regex("foo/**/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/another/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");
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
    use hyper::method::Method;
    use request::Request;
    use response::Response;

    let route_store = &mut Router::new();

    fn handler (_request: &Request, response: &mut Response) -> () {
        let _ = response.origin.write_all("hello from foo".as_bytes());
    };

    // issue #20178
    let handler_cast: fn(&Request, &mut Response) = handler;

    route_store.add_route(Method::Get, "/foo/:userid", handler_cast);
    route_store.add_route(Method::Get, "/bar", handler_cast);

    let route_result = route_store.match_route(&Method::Get, "/foo/4711").unwrap();
    let route = route_result.route;

    assert_eq!(route_result.param("userid"), "4711");

    //assert the route has identified the variable
    assert_eq!(route.variables.len(), 1);
    assert_eq!(route.variables["userid".to_string()], 0);

    let route_result = route_store.match_route(&Method::Get, "/bar/4711");
    assert!(route_result.is_none());

    let route_result = route_store.match_route(&Method::Get, "/foo");
    assert!(route_result.is_none());

    //ensure that this will work with commas too
    let route_result = route_store.match_route(&Method::Get, "/foo/123,456");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), "123,456");

    //ensure that this will work with spacing too
    let route_result = route_store.match_route(&Method::Get, "/foo/John%20Doe");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), "John%20Doe");
}

