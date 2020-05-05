use plugin::{Plugin, Pluggable};
use crate::request::Request;
use typemap::Key;
use crate::urlencoded::{Query, parse_uri};

struct QueryStringParser;

impl Key for QueryStringParser {
    type Value = Query;
}

impl<'mw, 'conn, D> Plugin<Request<'mw, 'conn, D>> for QueryStringParser {
    type Error = ();

    fn eval(req: &mut Request<'_, '_, D>) -> Result<Query, ()> {
        Ok(parse_uri(&req.origin.uri))
    }
}

pub trait QueryString {
    /// Extracts URL encoded data from the URL query string.
    /// # Examples
    /// ```{rust}
    /// #[macro_use] extern crate nickel;
    /// use nickel::{Nickel, HttpRouter, QueryString};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", middleware! { |req, res|
    ///         let query = req.query();
    ///         format!("Query: {:?}", query)
    ///     });
    /// }
    /// ```
    fn query(&mut self) -> &Query;
}

impl<'mw, 'conn, D> QueryString for Request<'mw, 'conn, D> {
    fn query(&mut self) -> &Query {
        self.get_ref::<QueryStringParser>()
            .ok()
            .expect("Bug: QueryStringParser returned None")
    }
}
