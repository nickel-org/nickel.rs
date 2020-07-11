use crate::{Response, MiddlewareResult, NickelError};
use hyper::header;
use crate::status::StatusCode;

pub trait Redirect: Sized {
    type Result;

    /// Redirect the response to a given target
    ///
    /// # Examples
    /// ```{rust}
    /// extern crate nickel;
    ///
    /// use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
    /// use nickel::extensions::Redirect;
    ///
    /// fn redirect<'mw, 'conn>(_: &mut Request<'mw, 'conn>, res: Response<'mw>) -> MiddlewareResult<'mw> {
    ///     return res.redirect("http://nickel.rs")
    /// }
    ///
    /// fn main() {
    ///     let mut server = Nickel::new();
    ///     server.get("/a", redirect);
    /// }
    /// ```
    fn redirect<T>(self, target: T) -> Self::Result
    where T: Into<String> {
        self.redirect_with(target, StatusCode::FOUND)
    }

    fn redirect_permanently<T>(self, target: T) -> Self::Result
    where T: Into<String> {
        self.redirect_with(target, StatusCode::MOVED_PERMANENTLY)
    }

    fn redirect_with<T>(self, target: T, status: StatusCode) -> Self::Result
    where T: Into<String>;
}

// TODO: rework this to return a Responder so it will work with the
// middleware macro
impl<D: Send + 'static + Sync> Redirect for Response<D> {
    type Result = MiddlewareResult<D>;

    fn redirect_with<T>(mut self, target: T, status: StatusCode) -> Self::Result
    where T: Into<String> {
        let header_value: header::HeaderValue = match target.into().parse() {
            Ok(v) => v,
            Err(e) => {
                return self.error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string());
            }
        };
        self.headers_mut().insert(header::LOCATION, header_value);

        let code = status.as_u16();
        if code < 300 || code >= 400 {
            self.error(StatusCode::INTERNAL_SERVER_ERROR, "redirect_with called with non-3xx status code")
        } else {
            self.send(status)
        }
    }
}
