//!Router asigns handlers to paths and resolves them per request

#[cfg(test)]
use http::method;
use http::method::Method;
use regex::Regex;
use std::collections::hashmap::HashMap;
use request::Request;
use response::Response;

/// A Route is the basic data structure that stores both the path
/// and the handler that gets executed for the route.
/// The path can contain variable pattern such as `user/:userid/invoices`
struct Route {
    pub path: String,
    pub method: Method,
    pub handler: fn(request: &Request, response: &mut Response),
    pub variables: HashMap<String, uint>,
    matcher: Regex
}

impl Clone for Route {
    fn clone(&self) -> Route {
        Route { 
            path: self.path.clone(), 
            method: self.method.clone(),
            handler: self.handler, 
            matcher: self.matcher.clone(),
            variables: self.variables.clone() 
        }
    }
}

/// A RouteResult is what the router returns when `match_route` is called.
/// It contains the matched `route` and also a `params` property holding
/// a HashMap with the keys being the variable names and the value being the
/// evaluated string
struct RouteResult<'a> {
    pub route: &'a Route,
    pub params: HashMap<String, String>
}

/// The PathUtils collects some small helper methods that operate on the path
struct PathUtils;

static REGEX_VAR_SEQ: Regex                 = regex!(r":([a-zA-Z0-9_-]*)");
static VAR_SEQ:&'static str                 = "[a-zA-Z0-9_-]*";
static VAR_SEQ_WITH_SLASH:&'static str      = "[/a-zA-Z0-9_-]*";
static VAR_SEQ_WITH_CAPTURE:&'static str    = "([a-zA-Z0-9_-]*)";
static REGEX_START:&'static str             = "^";
static REGEX_END:&'static str               = "$";


impl PathUtils {
    fn create_regex (route_path: &str) -> Regex {

        let updated_path = route_path.to_string()
                                     // first mark all double wildcards for replacement. We can't directly replace them
                                     // since the replacement does contain the * symbol as well, which would get overwritten
                                     // with the next replace call
                                     .replace("**", "___DOUBLE_WILDCARD___")
                                     // then replace the regular wildcard symbols (*) with the appropriate regex
                                     .replace("*", VAR_SEQ)
                                     // now replace the previously marked double wild cards (**)
                                     .replace("___DOUBLE_WILDCARD___", VAR_SEQ_WITH_SLASH);

        // then replace the variable symbols (:variable) with the appropriate regex
        let result = REGEX_START.to_string()
                                .append(REGEX_VAR_SEQ.replace_all(updated_path.as_slice(), VAR_SEQ_WITH_CAPTURE).as_slice())
                                .append(REGEX_END);

        match Regex::new(result.as_slice()) {
            Ok(re) => re,
            Err(err) => fail!("{}", err)
        }
    }

    fn get_variable_info (route_path: &str) -> HashMap<String, uint> {
        REGEX_VAR_SEQ.captures_iter(route_path)
             .enumerate()
             .map(|(i, matched)| (matched.at(1).to_string(), i))
             .collect()
    }
}

/// The Router's job is it to hold routes and to resolve them later against
/// concrete URLs

#[deriving(Clone)]
pub struct Router{
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new () -> Router {
        Router {
            routes: Vec::new()
        }
    }

    pub fn add_route (&mut self, method: Method, path: String, handler: fn(request: &Request, response: &mut Response)) -> () {
        let matcher = PathUtils::create_regex(path.as_slice());
        let variable_infos = PathUtils::get_variable_info(path.as_slice());
        let route = Route {
            path: path,
            method: method,
            matcher: matcher,
            handler: handler,
            variables: variable_infos
        };
        self.routes.push(route);
    }

    pub fn match_route<'a>(&'a self, method: Method, path: String) -> Option<RouteResult<'a>> {
        self.routes.iter().find(|item| item.method == method && item.matcher.is_match(path.as_slice()))
            .and_then(|route| {
                match route.matcher.captures(path.as_slice()) {
                    Some(captures) => {
                        let mut map = HashMap::new();
                        for (name, pos) in route.variables.iter() {
                            map.insert(name.to_string(), captures.at(pos + 1).to_string());
                        }
                        Some(RouteResult {
                            route: route,
                            params: map
                        })
                    },
                    None => Some(RouteResult{
                        route: route,
                        params: HashMap::new()
                    })
                }
            })
    }
}


#[test]
fn creates_map_with_var_variable_infos () {
    let map = PathUtils::get_variable_info("foo/:uid/bar/:groupid");
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&"uid".to_string()), &0);
    assert_eq!(map.get(&"groupid".to_string()), &1);
}

#[test]
fn creates_regex_with_captures () {
    let regex = PathUtils::create_regex("foo/:uid/bar/:groupid");
    let caps = regex.captures("foo/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");
    
    let regex = PathUtils::create_regex("foo/*/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");

    let regex = PathUtils::create_regex("foo/**/:uid/bar/:groupid");
    let caps = regex.captures("foo/test/another/4711/bar/5490").unwrap();

    assert_eq!(caps.at(1), "4711");
    assert_eq!(caps.at(2), "5490");
}

#[test]
fn creates_valid_regex_for_routes () {
    let regex1 = PathUtils::create_regex("foo/:uid/bar/:groupid");
    let regex2 = PathUtils::create_regex("foo/*/bar");
    let regex3 = PathUtils::create_regex("foo/**/bar");

    assert_eq!(regex1.is_match("foo/4711/bar/5490"), true);
    assert_eq!(regex1.is_match("foo/4711/bar"), false);

    assert_eq!(regex2.is_match("foo/4711/bar"), true);
    assert_eq!(regex2.is_match("foo/4711/4712/bar"), false);

    assert_eq!(regex3.is_match("foo/4711/bar"), true);
    assert_eq!(regex3.is_match("foo/4711/4712/bar"), true);
}


#[test]
fn can_match_var_routes () {
    let route_store = &mut Router::new();

    fn handler (request: &Request, response: &mut Response) -> () {
        response.origin.write("hello from foo".as_bytes()); 
    };

    route_store.add_route(method::Get, "/foo/:userid".to_string(), handler);
    route_store.add_route(method::Get, "/bar".to_string(), handler);
    
    let route_result = route_store.match_route(method::Get, "/foo/4711".to_string()).unwrap();
    let route = route_result.route;

    assert_eq!(route_result.params.get(&"userid".to_string()), &"4711".to_string());

    //assert the route has identified the variable
    assert_eq!(route.variables.len(), 1);
    assert_eq!(route.variables.get(&"userid".to_string()), &0);


    let route_result = route_store.match_route(method::Get, "/bar/4711".to_string());

    let result = match route_result {
        Some(res) => true,
        None => false
    };

    assert_eq!(result, false);

    let route_result = route_store.match_route(method::Get, "/foo".to_string());

    let result = match route_result{
        Some(res) => true,
        None => false
    };

    assert_eq!(result, false);
}
