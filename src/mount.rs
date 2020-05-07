use crate::nickel::Nickel;
use crate::request::Request;
use crate::response::Response;
use crate::middleware::{Continue, Middleware, MiddlewareResult};

pub trait Mountable<D> {
    fn mount<S: Into<String>, M: Middleware<B, D>>(&mut self, mount_point: S, middleware: M);
}

impl<D> Mountable<D> for Nickel<D>
where D: Send + Sync + 'static {
    /// A trait that makes mounting more convenient. Works the same as
    /// manually adding a `Mount` middleware.
    ///
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Nickel, StaticFilesHandler, Mountable};
    /// let mut server = Nickel::new();
    ///
    /// server.mount("/static_files/", StaticFilesHandler::new("/path/to/serve/"));
    /// ```
    ///
    /// # Panics
    /// Panics if mount_point does not have a leading and trailing slash.
    fn mount<S: Into<String>, M: Middleware<B, D>>(&mut self, mount_point: S, middleware: M) {
        self.utilize(Mount::new(mount_point, middleware));
    }
}

pub struct Mount<M> {
    mount_point: String,
    middleware: M
}

impl<M> Mount<M> {
    ///
    /// Creates a new middleware that mounts a middleware at a mount point.
    /// An incoming request that matches the mount point will be forwareded to
    /// the mounted middleware, but with the path rewritten so that the mount
    /// point appears to be the root from the perspective of the mounted
    /// middleware. This can be useful in combination with the
    /// `StaticFilesMiddleware`, for example.
    ///
    ///
    /// # Examples
    /// ```{rust}
    /// use nickel::{Nickel, StaticFilesHandler, Mount};
    /// let mut server = Nickel::new();
    ///
    /// server.utilize(
    ///     Mount::new("/static_files/",
    ///                StaticFilesHandler::new("/path/to/serve/")
    /// ));
    /// ```
    ///
    /// # Panics
    /// Panics if mount_point does not have a leading and trailing slash.
    pub fn new<S: Into<String>>(mount_point: S, middleware: M) -> Mount<M> {
        let mount_point: String = mount_point.into();
        match (mount_point.chars().last(), mount_point.chars().nth(0)) {
            (Some('/'), Some('/')) =>
                Mount {
                    mount_point: mount_point,
                    middleware: middleware
                },
            _ => panic!("Mount points must have a leading and trailing slash.")
        }
    }
}

impl<D, M: Middleware<B, D>> Middleware<B, D> for Mount<M> {
    fn invoke<'mw, 'conn>(&'mw self, req: &mut Request<'mw, B, D>, res: Response<'mw, D>)
                          -> MiddlewareResult<'mw, B, D> {
        // two clones in this method, there ought to be a way to avoid that
        let mut parts = req.origin.uri().clone().into_parts();
        match req.origin.uri().path_and_query() {
            Some(paq) if paq.starts_with(&*self.mount_point) => {
                let new_paq_str = format!(format!("/{}", &paq[self.mount_point.len()..]));
                parts.path_and_query = Some(new_paq_str.into());
            },
            _ => { return Ok(Continue(res)); }
        };
        
        let original = req.origin.uri().clone();
        *req.origin.uri_mut() = parts.into();
        let result = self.middleware.invoke(req, res);
        *req.origin.uri_mut() = original;
        result
    }
}
