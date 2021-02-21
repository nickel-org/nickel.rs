use crate::request::Request;
use crate::urlencoded::{Query, parse_uri};

// TODO: migration cleanup - Extensible does not support ShareMap, but TypeMap is not Sync+Send
// struct QueryStringParser;

// impl Key for QueryStringParser {
//     type Value = Query;
// }

// impl<D> Plugin<Request<D>> for QueryStringParser {
//     type Error = ();

//     fn eval(req: &mut Request<D>) -> Result<Query, ()> {
//         Ok(parse_uri(&req.origin.uri()))
//     }
// }

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
    fn query(&mut self) -> Query;
}

impl<D> QueryString for Request<D> {
    fn query(&mut self) -> Query {
        parse_uri(self.origin.uri())
        // TODO: migration cleanup - Extensible does not support ShareMap, but TypeMap is not Sync+Send
        // self.get_ref::<QueryStringParser>()
        //     .ok()
        //     .expect("Bug: QueryStringParser returned None")
    }
}
