use Request;
use hyper::header;

pub trait Referer {
    fn referer(&self) -> Option<&str>;
}

impl<'mw, 'server, D> Referer for Request<'mw, 'server, D> {
    /// Get the Request's referer header
    ///
    /// # Examples
    /// ```{rust}
    /// extern crate nickel;
    ///
    /// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
    /// use nickel::extensions::{Referer, Redirect};
    ///
    /// fn referer<'mw, 'conn>(req: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    ///     let back = req.referer().unwrap_or("http://nickel-org.github.io/");
    ///     return res.redirect(back)
    /// }
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", referer);
    /// }
    /// ```
    fn referer(&self) -> Option<&str> {
        self.origin.headers.get::<header::Referer>()
                           .map(|r| &***r)
    }
}
