use middleware::{Middleware, Continue, MiddlewareResult};

use request::Request;
use response::Response;
use router::HttpRouter;
use hyper::method::Method;
use hyper::status::StatusCode;
use router::{Matcher, FORMAT_PARAM};

/// A Route is the basic data structure that stores both the path
/// and the handler that gets executed for the route.
/// The path can contain variable pattern such as `user/:userid/invoices`
pub struct Route<D=()> {
    pub method: Method,
    pub handler: Box<Middleware<D> + Send + Sync + 'static>,
    matcher: Matcher
}

/// A RouteResult is what the router returns when `match_route` is called.
/// It contains the matched `route` and also a `params` property holding
/// a HashMap with the keys being the variable names and the value being the
/// evaluated string
pub struct RouteResult<'mw, D: 'mw> {
    pub route: &'mw Route<D>,
    params: Vec<(String, String)>
}

impl<'mw, D> RouteResult<'mw, D> {
    pub fn param(&self, key: &str) -> Option<&str> {
        for &(ref k, ref v) in &self.params {
            if k == &key {
                return Some(&v[..])
            }
        }

        // FIXME: should have a default format
        if key == FORMAT_PARAM {
            Some("")
        } else {
            None
        }
    }
}

/// The Router's job is it to hold routes and to resolve them later against
/// concrete URLs. The router is also a regular middleware and needs to be
/// added to the middleware stack with `server.utilize(router)`.
pub struct Router<D=()> {
    routes: Vec<Route<D>>,
}

impl<D=()> Router<D> {
    pub fn new() -> Router<D> {
        Router {
            routes: Vec::new()
        }
    }

    pub fn match_route<'mw>(&'mw self, method: &Method, path: &str) -> Option<RouteResult<'mw, D>> {
        self.routes
            .iter()
            .find(|item| item.method == *method && item.matcher.is_match(path))
            .map(|route|
                RouteResult {
                    params: extract_params(route, path),
                    route: route
                }
            )
    }
}

fn extract_params<D>(route: &Route<D>, path: &str) -> Vec<(String, String)> {
    match route.matcher.captures(path) {
        Some(captures) => {
            captures.iter_named()
                    .filter_map(|(name, subcap)| {
                        subcap.map(|cap| (name.to_string(), cap.to_string()))
                    })
                    .collect()
        }
        None => vec![]
    }
}

impl<D> HttpRouter<D> for Router<D> {
    fn add_route<M: Into<Matcher>, H: Middleware<D>>(&mut self, method: Method, matcher: M, handler: H) -> &mut Self {
        let route = Route {
            matcher: matcher.into(),
            method: method,
            handler: Box::new(handler),
        };

        self.routes.push(route);
        self
    }
}

impl<D: 'static> Middleware<D> for Router<D> {
    fn invoke<'mw, 'conn>(&'mw self, req: &mut Request<'mw, 'conn, D>, mut res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, D> {
        debug!("Router::invoke for '{:?}'", req.origin.uri);

        // Strip off the querystring when matching a route
        let route_result = req.path_without_query()
                              .and_then(|path| self.match_route(&req.origin.method, path));

        debug!("route_result.route.path: {:?}", route_result.as_ref().map(|r| r.route.matcher.path()));

        match route_result {
            Some(route_result) => {
                res.set(StatusCode::Ok);
                let handler = &route_result.route.handler;
                req.route_result = Some(route_result);
                handler.invoke(req, res)
            },
            None => Ok(Continue(res))
        }
    }
}

#[test]
fn creates_regex_with_captures () {
    let matcher: Matcher = "foo/:uid/bar/:groupid".into();
    let caps = matcher.captures("foo/4711/bar/5490").unwrap();

    assert_eq!(matcher.path(), "foo/:uid/bar/:groupid(\\.:format)?");
    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");

    let matcher: Matcher = "foo/*/:uid/bar/:groupid".into();
    let caps = matcher.captures("foo/test/4711/bar/5490").unwrap();

    assert_eq!(matcher.path(), "foo/*/:uid/bar/:groupid(\\.:format)?");
    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");

    let matcher: Matcher = "foo/**/:uid/bar/:groupid".into();
    let caps = matcher.captures("foo/test/another/4711/bar/5490").unwrap();

    assert_eq!(matcher.path(), "foo/**/:uid/bar/:groupid(\\.:format)?");
    assert_eq!(caps.at(1).unwrap(), "4711");
    assert_eq!(caps.at(2).unwrap(), "5490");

    let matcher: Matcher = "foo/**/:format/bar/:groupid".into();
    let caps = matcher.captures("foo/test/another/4711/bar/5490").unwrap();

    assert_eq!(matcher.path(), "foo/**/:format/bar/:groupid");
    assert_eq!(caps.name("format").unwrap(), "4711");
    assert_eq!(caps.name("groupid").unwrap(), "5490");
}

#[test]
fn creates_valid_regex_for_routes () {
    let multi_params: Matcher = "foo/:uid/bar/:groupid".into();

    assert!(multi_params.is_match("foo/4711/bar/5490"));
    assert!(multi_params.is_match("foo/4711/bar/5490?foo=true&bar=false"));
    assert!(!multi_params.is_match("foo/4711/bar"));
    assert!(!multi_params.is_match("foo/4711/bar?foo=true&bar=false"));
    assert!(multi_params.is_match("foo/4711/bar/test%20spacing"));
    assert!(multi_params.is_match("foo/4711/bar/5281?foo=test%20spacing&bar=false"));
    assert!(multi_params.is_match("foo/alice/bar/bob"));

    //ensure that this works with commas too
    assert!(multi_params.is_match("foo/4711/bar/5490,1234"));
    assert!(multi_params.is_match("foo/4711/bar/5490,1234?foo=true&bar=false"));
    assert!(!multi_params.is_match("foo/4711/bar"));
    assert!(!multi_params.is_match("foo/4711/bar?foo=1,2,3&bar=false"));

    //ensure that this works with hyphens too
    assert!(multi_params.is_match("foo/alice-anne/bar/bob-gates"));

    let single_asterisk: Matcher = "foo/*/bar".into();

    assert!(single_asterisk.is_match("foo/4711/bar"));
    assert!(!single_asterisk.is_match("foo/4711/barr"));
    assert!(single_asterisk.is_match("foo/4711/bar?foo=true&bar=false"));
    assert!(!single_asterisk.is_match("foo/4711/4712/bar"));
    assert!(!single_asterisk.is_match("foo/4711/4712/bar?foo=true&bar=false"));
    assert!(!single_asterisk.is_match("foo/alice/bar/bob"));
    assert!(single_asterisk.is_match("foo/alice/bar"));

    let double_asterisk: Matcher = "foo/**/bar".into();

    assert!(double_asterisk.is_match("foo/4711/bar"));
    assert!(double_asterisk.is_match("foo/4711/bar?foo=true&bar=false"));
    assert!(double_asterisk.is_match("foo/4711/4712/bar"));
    assert!(double_asterisk.is_match("foo/4711/4712/bar?foo=true&bar=false"));
    assert!(!double_asterisk.is_match("foo/alice/bar/bob"));
}

#[test]
fn can_match_var_routes () {
    let route_store = &mut Router::<()>::new();

    route_store.add_route(Method::Get, "/foo/:userid", middleware! { "hello from foo" });
    route_store.add_route(Method::Get, "/bar", middleware! { "hello from foo" });
    route_store.add_route(Method::Get, "/file/:format/:file", middleware! { "hello from foo" });

    let route_result = route_store.match_route(&Method::Get, "/foo/4711").unwrap();
    assert_eq!(route_result.param("userid"), Some("4711"));

    let route_result = route_store.match_route(&Method::Get, "/bar/4711");
    assert!(route_result.is_none());

    let route_result = route_store.match_route(&Method::Get, "/foo");
    assert!(route_result.is_none());

    // ensure that this will work with commas too
    let route_result = route_store.match_route(&Method::Get, "/foo/123,456");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), Some("123,456"));

    // ensure that this will work with spacing too
    let route_result = route_store.match_route(&Method::Get, "/foo/John%20Doe");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), Some("John%20Doe"));

    // check for optional format param
    let route_result = route_store.match_route(&Method::Get, "/foo/John%20Doe.json");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("userid"), Some("John%20Doe"));
    assert_eq!(route_result.param("format"), Some("json"));

    // ensure format works with queries
    let route_result = route_store.match_route(&Method::Get,
    "/foo/5490,1234.csv?foo=true&bar=false");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    // NOTE: `.param` doesn't cover query params currently
    assert_eq!(route_result.param("userid"), Some("5490,1234"));
    assert_eq!(route_result.param("format"), Some("csv"));

    // ensure format works with no format
    let route_result = route_store.match_route(&Method::Get,
                                               "/foo/5490,1234?foo=true&bar=false").unwrap();

    assert_eq!(route_result.param("format"), Some(""));

    // ensure format works if defined by user
    let route_result = route_store.match_route(&Method::Get, "/file/markdown/something?foo=true");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    // NOTE: `.param` doesn't cover query params currently
    assert_eq!(route_result.param("file"), Some("something"));
    assert_eq!(route_result.param("format"), Some("markdown"));
}

#[test]
fn params_lifetime() {
    let route_store = &mut Router::<()>::new();
    let handler = middleware! { "hello from foo" };

    route_store.add_route(Method::Get, "/file/:format/:file", handler);

    let route_result = route_store.match_route(&Method::Get, "/file/txt/manual");
    assert!(route_result.is_some());

    // Ensure two params can live without borrowck problems
    let route_result = route_result.unwrap();
    let format = route_result.param("format");
    let file = route_result.param("file");
    assert_eq!(format, Some("txt"));
    assert_eq!(file, Some("manual"));
}

#[test]
fn regex_path() {
    use regex::Regex;

    let route_store = &mut Router::<()>::new();

    let regex = Regex::new("/(foo|bar)").unwrap();
    route_store.add_route(Method::Get, regex, middleware! { "hello from foo" });

    let route_result = route_store.match_route(&Method::Get, "/foo");
    assert!(route_result.is_some());

    let route_result = route_store.match_route(&Method::Get, "/bar");
    assert!(route_result.is_some());

    let route_result = route_store.match_route(&Method::Get, "/bar?foo");
    assert!(route_result.is_some());

    let route_result = route_store.match_route(&Method::Get, "/baz");
    assert!(route_result.is_none());
}

#[test]
fn regex_path_named() {
    use regex::Regex;

    let route_store = &mut Router::<()>::new();

    let regex = Regex::new("/(?P<a>foo|bar)/b").unwrap();
    route_store.add_route(Method::Get, regex, middleware! { "hello from foo" });

    let route_result = route_store.match_route(&Method::Get, "/foo/b");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("a"), Some("foo"));

    let route_result = route_store.match_route(&Method::Get, "/bar/b");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("a"), Some("bar"));

    let route_result = route_store.match_route(&Method::Get, "/baz/b");
    assert!(route_result.is_none());
}

#[test]
fn ignores_querystring() {
    use regex::Regex;

    let route_store = &mut Router::<()>::new();

    let regex = Regex::new("/(?P<a>foo|bar)/b").unwrap();
    route_store.add_route(Method::Get, regex, middleware! { "hello from foo" });
    route_store.add_route(Method::Get, "/:foo", middleware! { "hello from foo" });

    // Should ignore the querystring
    let route_result = route_store.match_route(&Method::Get, "/moo?foo");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("foo"), Some("moo"));

    let route_result = route_store.match_route(&Method::Get, "/bar/b?foo");
    assert!(route_result.is_some());

    let route_result = route_result.unwrap();
    assert_eq!(route_result.param("a"), Some("bar"));
}
