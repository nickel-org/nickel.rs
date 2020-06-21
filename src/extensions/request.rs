use crate::Request;
use hyper::header;

pub trait Referer {
    fn referer(&self) -> Option<&str>;
}

impl<'mw, D> Referer for Request<'mw, D> {
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
        self.origin.headers().get(header::REFERER)
                           .and_then(|r| r.to_str().ok())
    }
}
