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
    /// #[macro_use] extern crate nickel;
    ///
    /// use nickel::{Nickel, HttpRouter};
    /// use nickel::extensions::{Referer, Redirect};
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", middleware! { |req, res|
    ///         let back = req.referer().unwrap_or("http://nickel.rs");
    ///         return res.redirect(back)
    ///     });
    /// }
    /// ```
    fn referer(&self) -> Option<&str> {
        self.origin.headers.get::<header::Referer>()
                           .map(|r| &***r)
    }
}
