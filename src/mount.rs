use nickel::Nickel;
use nickel_error::NickelError;
use request::Request;
use response::Response;
use middleware::{Continue, Middleware, MiddlewareResult};

use hyper::{StatusCode, Uri};

use std::mem;

pub trait Mountable<B, D> {
    fn mount<S: Into<String>, M: Middleware<B, D>>(&mut self, mount_point: S, middleware: M);
}

impl<B, D> Mountable<B, D> for Nickel<B, D>
where B: 'static, D: Send + Sync + 'static {
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

impl<B, D, M: Middleware<B, D>> Middleware<B, D> for Mount<M> {
    fn invoke<'mw>(&'mw self, req: &mut Request<'mw, B, D>, res: Response<'mw, B, D>)
                          -> MiddlewareResult<'mw, B, D> {
        // Todo: migration cleanup
        //
        // This is somewhat tricky. The new hyper::Uri does not
        // provide an easy way to rewrite the uri. It appears we'll
        // need to take apart the uri and put together a string with a
        // new path, then create a new Uri from that. Ugh. It may be
        // better to add a target field to nickel::Request that is
        // derived from req.origin.uri.
        // let subpath = match req.origin.uri {
        //     AbsolutePath(ref path) if path.starts_with(&*self.mount_point) => {
        //         AbsolutePath(format!("/{}", &path[self.mount_point.len()..]))
        //     },
        //     _ => return Ok(Continue(res))
        // };

        let subpath = if req.origin.uri().path().starts_with(&*self.mount_point) {
            let new_uri = req.origin.uri().as_ref().replacen(&*self.mount_point, "", 1);
            match new_uri.parse::<Uri>() {
                Ok(uri) => uri,
                Err(e) => {
                    // This implies a badly formatted mount point, so
                    // let's log a detailed error message. Ideally
                    // Mount::new() will prevent this, but stuff
                    // happens. If it does occur, we should treat it
                    // as a bug and modify Mount::new() to catch it.
                    let mce = "Mount consistency error";
                    error!("{}: {:?}, uri: {:?}, mount_point: {:?}",
                           mce, e, req.origin.uri(), self.mount_point);
                    return Err(NickelError::new(res, mce, StatusCode::InternalServerError));
                }
            }
        } else {
            return Ok(Continue(res));
        };

        // Todo: migration cleanup
        //
        // uri is now private, so this approach will neeed to be redesigned
        //
        // let original = mem::replace(&mut req.origin.uri, subpath);
        // let result = self.middleware.invoke(req, res);
        // req.origin.uri = original;
        panic!("mount not supported yet!");

        // result
    }
}
