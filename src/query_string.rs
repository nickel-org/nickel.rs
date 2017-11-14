use plugin::{Plugin, Pluggable};
use request::Request;
use typemap::Key;
use urlencoded::{Query, parse_uri};

struct QueryStringParser;

impl Key for QueryStringParser {
    type Value = Query;
}

impl<'mw, D> Plugin<Request<'mw, D>> for QueryStringParser {
    type Error = ();

    fn eval(req: &mut Request<D>) -> Result<Query, ()> {
        Ok(parse_uri(&req.origin.uri()))
    }
}

pub trait QueryString {
    /// Extracts URL encoded data from the URL query string.
    /// # Examples
    /// ```{rust}
    /// extern crate nickel;
    /// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult, QueryString};
    ///
    /// fn get_query<'mw>(req: &mut Request<'mw>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    ///     let query = req.query();
    ///     return res.send(format!("Query: {:?}", query))
    /// }
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", get_query);
    /// }
    /// ```
    fn query(&mut self) -> &Query;
}

impl<'mw, D> QueryString for Request<'mw, D> {
    fn query(&mut self) -> &Query {
        self.get_ref::<QueryStringParser>()
            .ok()
            .expect("Bug: QueryStringParser returned None")
    }
}
